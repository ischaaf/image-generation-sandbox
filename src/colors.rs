
use std::io::Write;

use crate::image_writer::write;

use self::regions::RegionFilter;

const R_SHIFT: u32 = 0;
const G_SHIFT: u32 = 8;
const B_SHIFT: u32 = 16;
const A_SHIFT: u32 = 24;

const A_MASK: u32 = 0xff << A_SHIFT;
const B_MASK: u32 = 0xff << B_SHIFT;
const G_MASK: u32 = 0xff << G_SHIFT;
const R_MASK: u32 = 0xff << R_SHIFT;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    pub fn new() -> Self {
        Self::from_combined(0x000000ff)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self{r, g, b, a}
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self{r, g, b, a: 0xff}
    }

    pub fn from_combined(val: u32) -> Self {
        Self {
            r: ((val & R_MASK) >> R_SHIFT) as u8,
            g: ((val & G_MASK) >> G_SHIFT) as u8,
            b: ((val & B_MASK) >> B_SHIFT) as u8,
            a: ((val & A_MASK) >> A_SHIFT) as u8,
        }
    }

    pub fn to_combined(&self) -> u32 {
        ((self.r as u32) << R_SHIFT) |
            ((self.g as u32) << G_SHIFT) |
            ((self.b as u32) << B_SHIFT) |
            ((self.a as u32) << A_SHIFT)
    }

    pub fn copy_from(&mut self, other: &Pixel) {
        self.r = other.r;
        self.g = other.g;
        self.b = other.b;
        self.a = other.a;
    }
}

pub struct Image {
    width: u32,
    height: u32,
    pub data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![Pixel::new(); (width*height) as usize],
        }
    }

    pub fn write(&self, name: &str) {
        let output_data = self.data.iter().map(|p| p.to_combined()).collect::<Vec<u32>>();
        write(name, output_data, self.width, self.height);
    }

    pub fn transform(&mut self, transformer: &dyn transformers::ImageTransformer) -> &mut Self {
        self.transform_region(transformer, &regions::NoF{})
    }

    pub fn transform_region(&mut self, transformer: &dyn transformers::ImageTransformer, region: &dyn RegionFilter) -> &mut Self {
        for (index, pixel) in self.data.iter_mut().enumerate() {
            let row = (index as u32) % self.width;
            let col = (index as u32) / self.width;
            if region.contains(row, col) {
                transformer.transform_pixel(row, col, pixel);
            }
        }
        self
    }
}

pub mod regions {
    pub trait RegionFilter {
        fn contains(&self, row: u32, col: u32) -> bool;
    }

    pub struct NoF {}

    impl RegionFilter for NoF {
        fn contains(&self, _: u32, _: u32) -> bool {
            true
        }
    }

    pub struct MutliRegion {
        rfs: Vec<Box<dyn RegionFilter>>,
    }

    impl MutliRegion {
        pub fn new(rfs: Vec<Box<dyn RegionFilter>>) -> Self {
            Self{rfs}
        }
    }

    impl RegionFilter for MutliRegion {
        fn contains(&self, row: u32, col: u32) -> bool {
            for rf in self.rfs.iter() {
                if rf.contains(row, col) {
                    return true;
                }
            }
            false
        }
    }

    pub struct TriangleF {
        p1: (i32, i32),
        p2: (i32, i32),
        p3: (i32, i32),
    }

    impl TriangleF {
        pub fn new(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> Self {
            Self{p1, p2, p3}
        }
        
        pub fn sign(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> i32 {
            (p1.0 - p3.0) * (p2.1 - p3.1) - (p2.0 - p3.0) * (p1.1 - p3.1)
        }
    }

    impl RegionFilter for TriangleF {
        fn contains(&self, row: u32, col: u32) -> bool {
            let pt = (row as i32, col as i32);
            let d1 = Self::sign(pt, self.p1, self.p2);
            let d2 = Self::sign(pt, self.p2, self.p3);
            let d3 = Self::sign(pt, self.p3, self.p1);

            let has_neg = (d1 < 0) || (d2 < 0) || (d3 < 0);
            let has_pos = (d1 > 0) || (d2 > 0) || (d3 > 0);

            !(has_neg && has_pos)
        }
    }

    pub struct Polygon {
        triangles: Vec<TriangleF>,
    }

    impl Polygon {
        pub fn square(bot_left: (i32, i32), size: i32) -> Self {
            Self{
                triangles: vec![
                    TriangleF::new(bot_left, (bot_left.0, bot_left.1 + size), (bot_left.0 + size, bot_left.1)),
                    TriangleF::new((bot_left.0 + size, bot_left.1 + size), (bot_left.0, bot_left.1 + size), (bot_left.0 + size, bot_left.1)),
                ],
            }
        }
    }

    impl RegionFilter for Polygon {
        fn contains(&self, row: u32, col: u32) -> bool {
            for triangle in self.triangles.iter() {
                if triangle.contains(row, col) {
                    return true;
                }
            }
            return false;
        }
    }

    pub struct RectangleF {
        bottom: u32,
        left: u32,
        width: u32,
        height: u32,
    }

    impl RectangleF {
        pub fn new(bottom: u32, left: u32, width: u32, height: u32) -> Self {
            Self{bottom, left, width, height}
        }
    }

    impl RegionFilter for RectangleF {
        fn contains(&self, row: u32, col: u32) -> bool {
            row >= self.bottom && row < self.bottom + self.height && col >= self.left && col < self.left + self.width
        }
    }
}

pub mod transformers {
    use super::Pixel;

    pub trait ImageTransformer {
        fn transform_pixel(&self, row: u32, col: u32, value: &mut Pixel);
    }

    pub struct SolidColorTransformer {
        color: Pixel,
    }

    impl SolidColorTransformer {
        pub fn new(color: Pixel) -> Self {
            Self{color}
        }
    }

    impl ImageTransformer for SolidColorTransformer {
        fn transform_pixel(&self, _row: u32, _col: u32, value: &mut Pixel) {
            value.copy_from(&self.color);
        }
    }

    pub struct CheckerboardTF {
        dim: u32,
        color_1: Pixel,
        color_2: Pixel,
    }

    impl CheckerboardTF {
        pub fn new(dim: u32, color_1: Pixel, color_2: Pixel) -> Self {
            Self{dim, color_1, color_2}
        }
    }

    impl ImageTransformer for CheckerboardTF {
        fn transform_pixel(&self, row: u32, col: u32, value: &mut Pixel) {
            let is_row_even = row / self.dim % 2 == 0;
            let is_col_even = col / self.dim % 2 == 0;
            if (is_row_even && is_col_even) || (!is_row_even && !is_col_even) {
                value.copy_from(&self.color_1);
            } else {
                value.copy_from(&self.color_2);
            }
        }
    }
}


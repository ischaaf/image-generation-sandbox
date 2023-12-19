use crate::image_writer::write;

use crate::regions::{NoF, Point, RegionFilter};
use crate::transformers::ImageTransformer;

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
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn new() -> Self {
        Self::from_combined(0x000000ff)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 0xff }
    }

    fn ensure_0_to_x(val: f32, x: f32) -> f32 {
        if val < 0f32 {
            val + x
        } else if val > x {
            val - x
        } else {
            val
        }
    }

    fn convert_temp_to_rgb(t_val: f32, t_1: f32, t_2: f32) -> u8 {
        if t_val * 6f32 < 1f32 {
            ((t_2 + (t_1 - t_2) * 6f32 * t_val) * 255f32).round() as u8
        } else if t_val * 2f32 < 1f32 {
            (t_1 * 255f32).round() as u8
        } else if t_val * 3f32 < 2f32 {
            ((t_2 + (t_1 - t_2) * (0.666 - t_val) * 6f32) * 255f32).round() as u8
        } else {
            (t_2 * 255f32).round() as u8
        }
    }

    pub fn hsla(h: f32, s: f32, l: f32, a: u8) -> Self {
        if s <= 0f32 + f32::EPSILON {
            let val = (l * 255f32) as u8;
            return Self::rgb(val, val, val);
        }
        let t1;
        if l < 0.5 {
            t1 = l * (1.0 + s);
        } else {
            t1 = l + s - l * s;
        }
        let t2 = (2f32 * l) - t1;
        let h_angle = h / 360f32;

        let t_r = Self::ensure_0_to_x(h_angle + 0.333, 1f32);
        let t_g = Self::ensure_0_to_x(h_angle, 1f32);
        let t_b = Self::ensure_0_to_x(h_angle - 0.333, 1f32);

        let r = Self::convert_temp_to_rgb(t_r, t1, t2);
        let g = Self::convert_temp_to_rgb(t_g, t1, t2);
        let b = Self::convert_temp_to_rgb(t_b, t1, t2);

        // println!("t_1: {}", t1);
        // println!("t_2: {}", t2);
        // println!("t_r: {}", t_r);
        // println!("t_g: {}", t_g);
        // println!("t_b: {}", t_b);
        Self::rgba(r, g, b, a)
    }

    pub fn to_hsla(&self) -> (f32, f32, f32, u8) {
        let r_norm = self.r as f32 / 255f32;
        let g_norm = self.g as f32 / 255f32;
        let b_norm = self.b as f32 / 255f32;

        let min = r_norm.min(g_norm).min(b_norm);
        let max = r_norm.max(g_norm).max(b_norm);

        let l = (min + max) / 2f32;

        let s;
        if (min - max).abs() <= f32::EPSILON {
            s = 0f32;
        } else if l <= 0.5f32 {
            s = (max - min) / (max + min);
        } else {
            s = (max - min) / (2f32 - max - min);
        }

        let h_norm;
        if (r_norm - max).abs() <= f32::EPSILON {
            h_norm = (g_norm - b_norm) / (max - min);
        } else if (g_norm - max).abs() <= f32::EPSILON {
            h_norm = 2f32 + (b_norm - r_norm) / (max - min);
        } else {
            h_norm = 4f32 + (r_norm - g_norm) / (max - min);
        }
        let h = Self::ensure_0_to_x(h_norm * 60f32, 360f32).round();

        (h, s, l, self.a)
    }

    pub fn complements(self) -> [Self; 2] {
        let (h, s, l, a) = self.to_hsla();
        let h = Self::ensure_0_to_x(h + 180.0, 360.0);
        [self, Self::hsla(h, s, l, a)]
    }

    pub fn analogous(self) -> [Self; 3] {
        let (h, s, l, a) = self.to_hsla();
        let h_lo = Self::ensure_0_to_x(h - 30.0, 360.0);
        let h_hi = Self::ensure_0_to_x(h + 30.0, 360.0);
        [Self::hsla(h_lo, s, l, a), self, Self::hsla(h_hi, s, l, a)]
    }

    pub fn triadic(self) -> [Self; 3] {
        let (h, s, l, a) = self.to_hsla();
        let h_lo = Self::ensure_0_to_x(h - 120.0, 360.0);
        let h_hi = Self::ensure_0_to_x(h + 120.0, 360.0);
        [Self::hsla(h_lo, s, l, a), self, Self::hsla(h_hi, s, l, a)]
    }

    pub fn tetradic(self) -> [Self; 4] {
        let (h, s, l, a) = self.to_hsla();
        let hl = Self::ensure_0_to_x(h - 90.0, 360.0);
        let hr = Self::ensure_0_to_x(h + 90.0, 360.0);
        let hc = Self::ensure_0_to_x(h + 180.0, 360.0);
        [
            self,
            Self::hsla(hr, s, l, a),
            Self::hsla(hc, s, l, a),
            Self::hsla(hl, s, l, a),
        ]
    }

    pub fn saturate(&self, s: f32) -> Self {
        let (h, _, l, a) = self.to_hsla();
        Self::hsla(h, s, l, a)
    }

    pub fn luminate(&self, l: f32) -> Self {
        let (h, s, _, a) = self.to_hsla();
        Self::hsla(h, s, l, a)
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
        ((self.r as u32) << R_SHIFT)
            | ((self.g as u32) << G_SHIFT)
            | ((self.b as u32) << B_SHIFT)
            | ((self.a as u32) << A_SHIFT)
    }

    pub fn copy_from(&mut self, other: &Pixel) {
        self.r = other.r;
        self.g = other.g;
        self.b = other.b;
        self.a = other.a;
    }

    pub fn set_alpha(mut self, value: u8) -> Self {
        self.a = value;
        self
    }

    pub fn blend(mut self, other: &Pixel) -> Self {
        let a = 255 - ((255 - self.a as u32) * (255 - other.a as u32) / 255) as u8;
        self.r = ((self.r as u32 * (255 - other.a as u32) + other.r as u32 * other.a as u32) / 255)
            as u8;
        self.g = ((self.g as u32 * (255 - other.a as u32) + other.g as u32 * other.a as u32) / 255)
            as u8;
        self.b = ((self.b as u32 * (255 - other.a as u32) + other.b as u32 * other.a as u32) / 255)
            as u8;
        self.a = a;

        self
    }
}

pub struct Image {
    pub size: Point,
    pub data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            size: Point {
                x: width,
                y: height,
            },
            data: vec![Pixel::new(); (width * height) as usize],
        }
    }

    pub fn get_pixel(&self, point: &Point) -> &Pixel {
        &self.data[point.to_linear(self.size.x) as usize]
    }

    pub fn write(&self, name: &str) {
        let output_data = self
            .data
            .iter()
            .map(|p| p.to_combined())
            .collect::<Vec<u32>>();
        write(name, output_data, self.size.x as u32, self.size.y as u32);
    }

    pub fn transform(&mut self, transformer: &mut dyn ImageTransformer) -> &mut Self {
        self.transform_region(transformer, &NoF {})
    }

    pub fn transform_region(
        &mut self,
        transformer: &mut dyn ImageTransformer,
        region: &dyn RegionFilter,
    ) -> &mut Self {
        let mut modifications: Vec<(usize, Pixel)> = vec![];
        for (index, pixel) in self.data.iter().enumerate() {
            let point = Point::from_linear(index as i32, self.size.x);
            if region.contains(&point) {
                modifications.push((index, transformer.transform_pixel(&point, pixel, &self)));
            }
        }

        for (index, new_pixel) in modifications {
            self.data[index].copy_from(&new_pixel);
        }
        self
    }
}

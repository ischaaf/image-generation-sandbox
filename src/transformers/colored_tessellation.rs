use crate::{
    images::{Image, Pixel},
    regions::{tess, Point, PointAnnotation, Region},
};

pub struct ColoredTessellationTF {
    tessellation: tess::PolygonTessellation,
    colors: Vec<Pixel>,
    use_descrete_colors: bool,
}

impl ColoredTessellationTF {
    pub fn new(tessellation: tess::PolygonTessellation, colors: Vec<Pixel>) -> Self {
        Self {
            tessellation,
            colors,
            use_descrete_colors: false,
        }
    }
    pub fn descrete(tessellation: tess::PolygonTessellation, colors: Vec<Pixel>) -> Self {
        Self {
            tessellation,
            colors,
            use_descrete_colors: true,
        }
    }
}

impl Region for ColoredTessellationTF {
    fn get_mutations(&self, image: &Image, mutations: &mut Vec<(Point, Pixel)>) {
        if self.tessellation.polygons.len() == 0 {
            return;
        }
        let full_bound_color = Pixel::hsl(120.0, 1.0, 0.5); // green
        let standard_bound_color = Pixel::hsl(240.0, 1.0, 0.5); // blue

        let mut last_y = self.tessellation.polygons[0].bounds().origin.y;
        let mut color_index = 0;
        let mut last_start = 0;

        for (index, poly) in self.tessellation.polygons.iter().enumerate() {
            if last_y != poly.bounds().origin.y {
                if last_start == 0 {
                    color_index = self.colors.len() - 1;
                    last_start = color_index;
                } else {
                    color_index = 0;
                    last_start = color_index;
                }
                last_y = poly.bounds().origin.y;
            }
            for point in poly.iter_points() {
                if point.x < 0 || point.x >= image.size.x || point.y < 0 || point.y >= image.size.y
                {
                    continue;
                }
                if self.use_descrete_colors {
                    let color = match point.annotation {
                        PointAnnotation::Regular => self.colors[index].clone(),
                        PointAnnotation::StandardEdge => self.colors[index].clone().hue_shift(0.0),
                        PointAnnotation::FullEdge => self.colors[index].clone().hue_shift(0.0),
                    };
                    match point.annotation {
                        PointAnnotation::Regular => mutations.push((point, color)),
                        PointAnnotation::StandardEdge => mutations.push((point, color)),
                        PointAnnotation::FullEdge => mutations.push((point, color)),
                    }
                } else {
                    mutations.push((point, self.colors[color_index].clone()));
                }
            }
            color_index = (color_index + 1) % self.colors.len();
        }
    }
}

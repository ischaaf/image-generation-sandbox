use crate::{
    images::{Image, Pixel},
    regions::{tess, Point, RegionFilter},
};

use super::ImageTransformer;

pub struct ColoredTessellationTF {
    tessellation: tess::PolygonTessellation,
    colors: Vec<Pixel>,
}

impl ImageTransformer for ColoredTessellationTF {
    fn transform_pixel(&mut self, point: &Point, value: &Pixel, _image: &Image) -> Pixel {
        let mut last_y = 0;
        let mut color_index = 0;
        let mut last_start = 0;
        for poly in self.tessellation.polygons.iter() {
            if last_y != poly.bounding_box.origin.y {
                if last_start == 0 {
                    color_index = self.colors.len() - 1;
                    last_start = color_index;
                } else {
                    color_index = 0;
                    last_start = color_index;
                }
                last_y = poly.bounding_box.origin.y;
            }
            if poly.contains(point) {
                return value.clone().blend(&self.colors[color_index]);
            }
        }
        value.clone()
    }
}

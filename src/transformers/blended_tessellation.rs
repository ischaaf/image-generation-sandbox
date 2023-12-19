use std::collections::HashMap;

use crate::{
    images::{Image, Pixel},
    regions::{tess, Point, RegionFilter},
};

use super::ImageTransformer;

pub struct BlendedTessellationTF {
    tessellation: tess::PolygonTessellation,
    color_cache: HashMap<usize, Pixel>,
}

impl BlendedTessellationTF {
    pub fn new(
        make_polygon_fn: tess::MakePolygonFn,
        center_fn: tess::CenterFn,
        side_len: i32,
        bounds: &Point,
    ) -> Self {
        let tessellation =
            tess::PolygonTessellation::new(make_polygon_fn, center_fn, side_len, bounds);
        Self {
            tessellation,
            color_cache: HashMap::new(),
        }
    }
}

impl ImageTransformer for BlendedTessellationTF {
    fn transform_pixel(&mut self, point: &Point, value: &Pixel, image: &Image) -> Pixel {
        for (index, poly) in self.tessellation.polygons.iter().enumerate() {
            if !poly.contains(point) {
                continue;
            }
            if let Some(pixel) = self.color_cache.get(&index) {
                return pixel.clone();
            } else {
                let bbox = &poly.bounding_box;
                let mut pixels_to_blind: Vec<Pixel> = vec![];
                for x in bbox.origin.x..(bbox.origin.x + bbox.size.x) {
                    if x < 0 || x >= image.size.x {
                        continue;
                    }
                    for y in bbox.origin.y..(bbox.origin.y + bbox.size.y) {
                        if y < 0 || y >= image.size.y {
                            continue;
                        }
                        let point_to_blend = Point::new(x as i32, y as i32);
                        if poly.contains(&point_to_blend) {
                            pixels_to_blind.push(image.get_pixel(&point_to_blend).clone());
                        }
                    }
                }

                let mut alpha: u8 = (255 / pixels_to_blind.len()) as u8;
                if alpha == 0 {
                    alpha = 1;
                }
                let alpha = alpha;

                let mut result = pixels_to_blind[0].clone().set_alpha(alpha);
                for pixel_to_blind in pixels_to_blind.iter().skip(1) {
                    result = result.blend(&pixel_to_blind.clone().set_alpha(alpha));
                }
                let to_ret = result.clone();
                self.color_cache.insert(index, result);
                return to_ret;
            }
            // if hex.contains(point) {
            //     return value.clone().blend(color);
            // }
        }
        value.clone()
    }
}

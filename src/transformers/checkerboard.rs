use crate::{
    images::{Image, Pixel},
    regions::Point,
};

use super::ImageTransformer;

pub struct CheckerboardTF {
    dim: i32,
    color_1: Pixel,
    color_2: Pixel,
}

impl CheckerboardTF {
    pub fn new(dim: i32, color_1: Pixel, color_2: Pixel) -> Self {
        Self {
            dim,
            color_1,
            color_2,
        }
    }
}

impl ImageTransformer for CheckerboardTF {
    fn transform_pixel(&mut self, point: &Point, value: &Pixel, _image: &Image) -> Pixel {
        let is_y_even = point.y / self.dim % 2 == 0;
        let is_x_even = point.x / self.dim % 2 == 0;
        if (is_y_even && is_x_even) || (!is_y_even && !is_x_even) {
            self.color_1.clone()
        } else {
            self.color_2.clone()
        }
    }
}

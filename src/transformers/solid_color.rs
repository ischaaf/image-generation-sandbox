use crate::{
    images::{Image, Pixel},
    regions::Point,
};

use super::ImageTransformer;

pub struct SolidColorTransformer {
    color: Pixel,
}

impl SolidColorTransformer {
    pub fn new(color: Pixel) -> Self {
        Self { color }
    }
}

impl ImageTransformer for SolidColorTransformer {
    fn transform_pixel(&mut self, _point: &Point, value: &Pixel, _image: &Image) -> Pixel {
        self.color.clone()
    }
}

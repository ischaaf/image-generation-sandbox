use crate::{
    curves::CurveFn,
    images::{Image, Pixel},
    regions::Point,
};

use super::ImageTransformer;

pub struct ColorWaveTF {
    hue_fn: CurveFn,
    sat_fn: CurveFn,
    lum_fn: CurveFn,
}

impl ColorWaveTF {
    pub fn new(hue_fn: CurveFn, sat_fn: CurveFn, lum_fn: CurveFn) -> Self {
        Self {
            hue_fn,
            sat_fn,
            lum_fn,
        }
    }
}

impl ImageTransformer for ColorWaveTF {
    fn transform_pixel(&mut self, point: &Point, _value: &Pixel, image: &Image) -> Pixel {
        let hue = (self.hue_fn)(point, &image.size);
        let sat = (self.sat_fn)(point, &image.size);
        let lum = (self.lum_fn)(point, &image.size);
        Pixel::hsla(hue, sat, lum, 0xff)
    }
}

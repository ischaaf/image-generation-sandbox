use crate::{
    curves::CurveFn,
    images::{Image, Pixel},
    regions::{Point, Region},
};

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

impl Region for ColorWaveTF {
    fn get_mutations(&self, image: &Image, mutations: &mut Vec<(Point, Pixel)>) {
        for x in 0..image.size.x {
            for y in 0..image.size.y {
                let point = Point::new(x, y);
                let hue = (self.hue_fn)(&point, &image.size);
                let sat = (self.sat_fn)(&point, &image.size);
                let lum = (self.lum_fn)(&point, &image.size);
                let pixel = Pixel::hsla(hue, sat, lum, 0xff);
                mutations.push((point, pixel));
            }
        }
    }
}

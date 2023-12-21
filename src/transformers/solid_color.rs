use crate::{
    images::{Image, Pixel},
    regions::{Point, Polygon, Region},
};

pub struct SolidColorPolygon {
    polygon: Polygon,
    color: Pixel,
}

impl SolidColorPolygon {
    pub fn new(polygon: Polygon, color: Pixel) -> Self {
        Self { polygon, color }
    }
}

impl Region for SolidColorPolygon {
    fn get_mutations(&self, image: &Image, mutations: &mut Vec<(Point, Pixel)>) {
        for point in self.polygon.iter_points() {
            mutations.push((point, self.color.clone()));
        }
    }
}

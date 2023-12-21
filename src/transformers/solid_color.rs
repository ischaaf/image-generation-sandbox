use crate::{
    images::{Image, Pixel},
    regions::{Point, PointAnnotation, Polygon, Region},
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
        let full_bound_color = Pixel::hsl(0.0, 1.0, 0.0);
        let standard_bound_color = Pixel::hsl(180.0, 1.0, 1.0);
        for point in self.polygon.iter_points() {
            println!("Found pixel with annotation: {:?}", point.annotation);
            let color = match point.annotation {
                PointAnnotation::Regular => self.color.clone(),
                PointAnnotation::FullEdge => full_bound_color.clone(),
                PointAnnotation::StandardEdge => standard_bound_color.clone(),
            };
            mutations.push((point, color));
        }
    }
}

use crate::{
    images::{Image, Pixel},
    regions::{Orientation, Point, Rectangle, Region, Triangle},
};

pub struct SolidTriangle {
    triangle: Triangle,
    color: Pixel,
}

impl SolidTriangle {
    pub fn new(p1: Point, p2: Point, p3: Point, orientation: Orientation, color: Pixel) -> Self {
        Self {
            triangle: Triangle::new_standard(p1, p2, p3, orientation),
            color,
        }
    }

    pub fn new_any(p1: Point, p2: Point, p3: Point, color: Pixel) -> Self {
        Self {
            triangle: Triangle::new(p1, p2, p3),
            color,
        }
    }
}

impl Region for SolidTriangle {
    fn get_mutations(&self, _image: &Image, mutations: &mut Vec<(Point, Pixel)>) {
        for point in self.triangle.iter_points() {
            mutations.push((point, self.color.clone()))
        }
    }
}

pub struct SolidRectangle {
    rectangle: Rectangle,
    color: Pixel,
}

impl SolidRectangle {
    pub fn new(origin: Point, size: Point, color: Pixel) -> Self {
        Self {
            rectangle: Rectangle::new(origin, size),
            color,
        }
    }
}

impl Region for SolidRectangle {
    fn get_mutations(&self, _image: &Image, mutations: &mut Vec<(Point, Pixel)>) {
        for point in self.rectangle.iter_points() {
            mutations.push((point, self.color.clone()))
        }
    }
}

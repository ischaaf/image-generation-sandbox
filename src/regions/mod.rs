mod point;
mod polygon;
mod rectangle;
mod tessellation;
mod triangle;

pub const ROOT_3: f64 = 1.732;

pub use point::Point;
pub use polygon::Polygon;
pub use rectangle::{Rectangle, RectangleIterator};
pub use triangle::{Orientation, StandardTriangle, StandardTriangleIterator, Triangle};

use crate::images::{Image, Pixel};

pub trait Region {
    fn get_mutations(&self, image: &Image, mutations: &mut Vec<(Point, Pixel)>);
}

pub mod tess {
    pub use super::tessellation::CenterFn;
    pub use super::tessellation::MakePolygonFn;
    pub use super::tessellation::PolygonTessellation;
}

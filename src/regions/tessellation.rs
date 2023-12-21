use super::{Point, Polygon, Rectangle};

pub type MakePolygonFn = fn(bounds: &Rectangle) -> Polygon;
pub type CenterFn =
    fn(poly_index: &Point, normalized_bounds: &Rectangle, focal_point: &Point) -> Rectangle;

pub struct PolygonTessellation {
    pub polygons: Vec<Polygon>,
}

impl PolygonTessellation {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        Self { polygons }
    }
}

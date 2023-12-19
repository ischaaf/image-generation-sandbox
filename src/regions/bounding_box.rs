use super::{point::Point, RegionFilter};

#[derive(Clone)]
pub struct BBox {
    pub origin: Point,
    pub size: Point,
}

impl BBox {
    pub fn new(origin: Point, size: Point) -> Self {
        Self { origin, size }
    }

    pub fn from_bounds(bot_left: Point, top_right: Point) -> Self {
        Self {
            origin: bot_left.clone(),
            size: Point::new(top_right.x - bot_left.x, top_right.y - bot_left.y),
        }
    }
}
impl RegionFilter for BBox {
    fn contains(&self, point: &Point) -> bool {
        let x_in = point.x >= self.origin.x && point.x <= self.origin.x + self.size.x;
        let y_in = point.y >= self.origin.y && point.y <= self.origin.y + self.size.y;
        x_in && y_in
    }
}

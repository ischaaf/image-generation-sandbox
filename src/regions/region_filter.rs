use super::point::Point;

pub trait RegionFilter {
    fn contains(&self, point: &Point) -> bool;
}

pub struct NoF {}

impl RegionFilter for NoF {
    fn contains(&self, _: &Point) -> bool {
        true
    }
}

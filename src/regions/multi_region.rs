use super::{point::Point, region_filter::RegionFilter};

pub struct MutliRegion {
    rfs: Vec<Box<dyn RegionFilter>>,
}

impl MutliRegion {
    pub fn new(rfs: Vec<Box<dyn RegionFilter>>) -> Self {
        Self { rfs }
    }
}

impl RegionFilter for MutliRegion {
    fn contains(&self, point: &Point) -> bool {
        for rf in self.rfs.iter() {
            if rf.contains(point) {
                return true;
            }
        }
        false
    }
}

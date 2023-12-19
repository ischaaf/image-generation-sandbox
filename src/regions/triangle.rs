use super::{point::Point, region_filter::RegionFilter};

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Self { p1, p2, p3 }
    }

    pub fn sign(p1: &Point, p2: &Point, p3: &Point) -> i32 {
        (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
    }

    pub fn bot_left(&self) -> Point {
        let x = self.p1.x.min(self.p2.x).min(self.p3.x);
        let y = self.p1.y.min(self.p2.y).min(self.p3.y);
        Point::new(x, y)
    }

    pub fn top_right(&self) -> Point {
        let x = self.p1.x.max(self.p2.x).max(self.p3.x);
        let y = self.p1.y.max(self.p2.y).max(self.p3.y);
        Point::new(x, y)
    }
}

impl RegionFilter for Triangle {
    fn contains(&self, point: &Point) -> bool {
        let pt = point;
        let d1 = Self::sign(pt, &self.p1, &self.p2);
        let d2 = Self::sign(pt, &self.p2, &self.p3);
        let d3 = Self::sign(pt, &self.p3, &self.p1);

        let has_neg = (d1 < 0) || (d2 < 0) || (d3 < 0);
        let has_pos = (d1 > 0) || (d2 > 0) || (d3 > 0);

        !(has_neg && has_pos)
    }
}

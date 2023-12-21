use super::Point;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Rectangle {
    pub origin: Point,
    pub size: Point,
}

impl Rectangle {
    pub fn new(origin: Point, size: Point) -> Self {
        Self { origin, size }
    }

    pub fn top_right(&self) -> Point {
        Point::new(self.origin.x + self.size.x, self.origin.y + self.size.y)
    }

    pub fn normal(size: Point) -> Self {
        Self {
            origin: Point::new(0, 0),
            size,
        }
    }

    pub fn from_bounds(bot_left: Point, top_right: Point) -> Self {
        Self {
            origin: bot_left.clone(),
            size: Point::new(top_right.x - bot_left.x, top_right.y - bot_left.y),
        }
    }

    pub fn center(&self) -> Point {
        Point::new(
            self.origin.x + self.size.x / 2,
            self.origin.y + self.size.y / 2,
        )
    }

    pub fn center_on(mut self, new_center: &Point) -> Self {
        self.origin.x = new_center.x - self.size.x / 2;
        self.origin.y = new_center.y - self.size.y / 2;
        self
    }

    pub fn extent(&self) -> Point {
        Point::new(self.origin.x + self.size.x, self.origin.y + self.size.y)
    }

    pub fn contains(&self, point: &Point) -> bool {
        let ext = self.extent();
        point.x >= self.origin.x && point.x < ext.x && point.y >= self.origin.y && point.y < ext.y
    }

    pub fn union(&mut self, other: &Rectangle) {
        let my_extent = self.extent();
        let other_extent = other.extent();
        self.origin.x = self.origin.x.min(other.origin.x);
        self.origin.y = self.origin.y.min(other.origin.y);

        let new_extent_x = my_extent.x.max(other_extent.x);
        let new_extent_y = my_extent.y.max(other_extent.y);

        self.size.x = new_extent_x - self.origin.x;
        self.size.y = new_extent_y - self.origin.y;
    }

    pub fn iter_points<'a>(&'a self) -> RectangleIterator<'a> {
        RectangleIterator {
            rectangle: self,
            cur_point: self.origin.clone(),
            top_right: self.top_right(),
        }
    }

    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(self.origin.clone(), self.size.clone())
    }
}

#[derive(PartialEq, Eq)]
pub struct RectangleIterator<'a> {
    rectangle: &'a Rectangle,
    cur_point: Point,
    top_right: Point,
}

impl<'a> Iterator for RectangleIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.cur_point.clone();
        if result.y >= self.top_right.y {
            return None;
        }

        self.cur_point.x += 1;
        if self.cur_point.x >= self.top_right.x {
            self.cur_point.x = self.rectangle.origin.x;
            self.cur_point.y += 1;
        }

        Some(result)
    }
}

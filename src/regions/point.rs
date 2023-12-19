#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_linear(&self, max_x: i32) -> i32 {
        self.y * max_x + self.x
    }

    pub fn from_linear(value: i32, max_x: i32) -> Self {
        Self {
            x: value % max_x,
            y: value / max_x,
        }
    }

    pub fn distance(&self, other: &Point) -> f32 {
        f32::sqrt(self.distance_square(other) as f32)
    }

    pub fn distance_square(&self, other: &Point) -> u32 {
        self.x.abs_diff(other.x).pow(2) + self.y.abs_diff(other.y).pow(2)
    }
}

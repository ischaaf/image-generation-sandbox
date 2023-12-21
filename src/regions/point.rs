use std::fmt::Debug;

const ORIGIN: Point = Point::new(0, 0);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
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

    pub fn dot(&self, other: &Point) -> i32 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f32 {
        self.distance(&ORIGIN)
    }

    pub fn distance(&self, other: &Point) -> f32 {
        f32::sqrt(self.distance_square(other) as f32)
    }

    pub fn distance_square(&self, other: &Point) -> u32 {
        self.x.abs_diff(other.x).pow(2) + self.y.abs_diff(other.y).pow(2)
    }

    pub fn to_float(&self) -> PointFloat {
        PointFloat::new(self.x as f32, self.y as f32)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone)]
pub struct PointFloat {
    pub x: f32,
    pub y: f32,
}

impl PointFloat {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn truncate(&self) -> Point {
        Point::new(self.x.trunc() as i32, self.y.trunc() as i32)
    }

    pub fn round(&self) -> Point {
        Point::new(self.x.round() as i32, self.y.round() as i32)
    }

    pub fn distance(&self, other: &PointFloat) -> f32 {
        f32::sqrt(self.distance_square(other) as f32)
    }

    pub fn distance_square(&self, other: &PointFloat) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    pub fn inv_slope(&self, other: &PointFloat) -> f32 {
        (other.x - self.x) / (other.y - self.y)
    }
}

impl Debug for PointFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2},{:.2})", self.x, self.y)
    }
}

use std::fmt::Debug;

use super::{
    point::{Point, PointFloat},
    Rectangle,
};

const DISABLE_SKIP: bool = false;

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct StandardTriangle {
    p1: Point,
    p2: Point,
    p3: Point,
    orientation: Orientation,
    is_transposed: bool,
}

const PERMUTATIONS: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

const ORIENTATIONS: [Orientation; 4] = [
    Orientation::Up,
    Orientation::Down,
    Orientation::Right,
    Orientation::Left,
];

const PI: f32 = 3.14159;

impl StandardTriangle {
    pub fn new(p1: Point, p2: Point, p3: Point, orientation: Orientation) -> Self {
        assert!(Self::valid_orientation(&p1, &p2, &p3, orientation));
        Self {
            p1,
            p2,
            p3,
            orientation,
            is_transposed: false,
        }
    }

    pub fn valid_orientation(p1: &Point, p2: &Point, p3: &Point, orientation: Orientation) -> bool {
        match orientation {
            Orientation::Up => p1.y == p2.y && p3.y > p1.y && p1.x < p2.x,
            Orientation::Down => p1.y == p2.y && p3.y < p1.y && p1.x < p2.x,
            Orientation::Left => p1.x == p2.x && p3.x < p1.x && p1.y < p2.y,
            Orientation::Right => p1.x == p2.x && p3.x > p1.x && p1.y < p2.y,
        }
    }

    pub fn try_new(p1: &Point, p2: &Point, p3: &Point) -> Option<Self> {
        let pts = [p1, p2, p3];
        for orientation in ORIENTATIONS.iter() {
            for perm in PERMUTATIONS.iter() {
                let tp1 = pts[perm[0]];
                let tp2 = pts[perm[1]];
                let tp3 = pts[perm[2]];
                if Self::valid_orientation(tp1, tp2, tp3, *orientation) {
                    return Some(Self::new(
                        tp1.clone(),
                        tp2.clone(),
                        tp3.clone(),
                        *orientation,
                    ));
                }
            }
        }
        None
    }

    pub fn height(&self) -> i32 {
        match self.orientation {
            Orientation::Up | Orientation::Down => self.p3.y.abs_diff(self.p2.y) as i32,
            Orientation::Left | Orientation::Right => self.p3.x.abs_diff(self.p2.x) as i32,
        }
    }

    pub fn bounds(&self) -> Rectangle {
        Rectangle::from_bounds(
            Point::new(
                self.p1.x.min(self.p2.x).min(self.p3.x),
                self.p1.y.min(self.p2.y).min(self.p3.y),
            ),
            Point::new(
                self.p1.x.max(self.p2.x).max(self.p3.x),
                self.p1.y.max(self.p2.y).max(self.p3.y),
            ),
        )
    }

    pub fn transpose(&self) -> Self {
        assert!(!self.is_transposed);
        match self.orientation {
            Orientation::Up => Self {
                p1: self.p1.clone(),
                p2: self.p2.clone(),
                p3: self.p3.clone(),
                orientation: self.orientation,
                is_transposed: true,
            },
            Orientation::Down => Self {
                p1: self.p1.clone(),
                p2: self.p2.clone(),
                p3: Point::new(self.p3.x, self.p3.y + 2 * self.height()),
                orientation: self.orientation,
                is_transposed: true,
            },
            Orientation::Left => Self {
                p1: Point::new(self.p1.y, self.p1.x),
                p2: Point::new(self.p2.y, self.p2.x),
                p3: Point::new(self.p3.y, self.p3.x + 2 * self.height()),
                orientation: self.orientation,
                is_transposed: true,
            },
            Orientation::Right => Self {
                p1: Point::new(self.p1.y, self.p1.x),
                p2: Point::new(self.p2.y, self.p2.x),
                p3: Point::new(self.p3.y, self.p3.x),
                orientation: self.orientation,
                is_transposed: true,
            },
        }
    }

    pub fn untranspose(&self, point: Point) -> Point {
        assert!(self.is_transposed);
        match self.orientation {
            Orientation::Up => point,
            Orientation::Down => {
                let mut p = point;
                p.y -= (2 * p.y.abs_diff(self.p1.y)) as i32;
                p
            }
            Orientation::Left => {
                let dist = point.y.abs_diff(self.p1.y) as i32;
                let res = Point::new(point.y - 2 * dist, point.x);
                res
            }
            Orientation::Right => Point::new(point.y, point.x),
        }
    }

    pub fn iter_points<'a>(&'a self) -> StandardTriangleIterator<'a> {
        StandardTriangleIterator::new(self)
    }
}

impl Debug for StandardTriangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "p1={:?}, p2={:?}, p3={:?}, orientation: {:?}",
            self.p1, self.p2, self.p3, self.orientation
        )
    }
}

pub struct StandardTriangleIterator<'a> {
    triangle: &'a StandardTriangle,
    transposed_triange: StandardTriangle,
    inv_slope_left: f32,
    inv_slope_right: f32,
    cur_left: PointFloat,
    cur_right: PointFloat,
    cur_point: Point,
    done: bool,
}

impl<'a> StandardTriangleIterator<'a> {
    fn new(triangle: &'a StandardTriangle) -> Self {
        println!(
            "iterating points in triangle: {:?}, {:?}, {:?}",
            triangle.p1, triangle.p2, triangle.p3
        );
        let transposed_triange = triangle.transpose();

        let p3_f = transposed_triange.p3.to_float();
        let cur_left = transposed_triange.p1.to_float();
        let cur_right = transposed_triange.p2.to_float();
        let cur_point = transposed_triange.p1.clone();

        let mut result = Self {
            triangle,
            transposed_triange,
            inv_slope_left: cur_left.inv_slope(&p3_f),
            inv_slope_right: -1.0 * p3_f.inv_slope(&cur_right),
            cur_left,
            cur_right,
            cur_point,
            done: false,
        };

        result.fast_forward_to_start();

        result
    }

    fn fast_forward_to_start(&mut self) {
        if self.skip_bot() {
            self.advance_y();
        } else if self.skip_left() {
            self.advance_x();
        }
    }

    fn advance_x(&mut self) {
        self.cur_point.x += 1;
        let skip = match self.skip_right() {
            false => 0.0,
            true => -1.0,
        };
        if self.cur_point.x as f32 > (self.cur_right.x.round() + skip) {
            self.advance_y();
        }
    }

    fn advance_y(&mut self) {
        self.cur_left.y += 1.0;
        self.cur_left.x += self.inv_slope_left;

        self.cur_right.y += 1.0;
        self.cur_right.x -= self.inv_slope_right;

        self.cur_point = self.cur_left.round();
        if self.skip_left() {
            self.cur_point.x += 1;
        }

        let skip = match self.skip_top() {
            false => 0.0,
            true => -1.0,
        };
        if self.cur_left.x > (self.cur_right.x + skip) {
            self.done = true;
        }
    }

    fn skip_left(&self) -> bool {
        if DISABLE_SKIP {
            return false;
        }
        return match self.triangle.orientation {
            Orientation::Up => self.triangle.p3.x > self.triangle.p1.x,
            Orientation::Down => self.triangle.p3.x < self.triangle.p1.x,
            Orientation::Left => false,
            Orientation::Right => false,
        };
    }

    fn skip_right(&self) -> bool {
        if DISABLE_SKIP {
            return false;
        }
        return match self.triangle.orientation {
            Orientation::Up => self.triangle.p3.x < self.triangle.p2.x,
            Orientation::Down => self.triangle.p3.x > self.triangle.p2.x,
            Orientation::Left => true,
            Orientation::Right => true,
        };
    }

    fn skip_top(&self) -> bool {
        if DISABLE_SKIP {
            return false;
        }
        match self.triangle.orientation {
            Orientation::Up => true,
            Orientation::Down => false,
            Orientation::Left => false,
            Orientation::Right => false,
        }
    }

    fn skip_bot(&self) -> bool {
        if DISABLE_SKIP {
            return false;
        }
        match self.triangle.orientation {
            Orientation::Up => false,
            Orientation::Down => true,
            Orientation::Left => false,
            Orientation::Right => false,
        }
    }
}

impl<'a> Iterator for StandardTriangleIterator<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.transposed_triange.untranspose(self.cur_point.clone());
        println!("Point iter found: {:?}", result);
        self.advance_x();

        Some(result)
    }
}

pub struct Triangle {
    t1: StandardTriangle,
    t2: Option<StandardTriangle>,
}

fn prepare_point_order(p1: Point, p2: Point, p3: Point) -> (Point, Point, Point) {
    let mut pts = [p1, p2, p3];
    if pts[0].y > pts[1].y {
        pts.swap(0, 1);
    }
    if pts[1].y > pts[2].y {
        pts.swap(1, 2);
    }
    if pts[0].y > pts[1].y {
        pts.swap(0, 1);
    }

    let [p3, p1, p2] = pts;
    (p1, p2, p3)
}

impl Triangle {
    pub fn new_standard(p1: Point, p2: Point, p3: Point, orientation: Orientation) -> Self {
        let t1 = StandardTriangle::new(p1, p2, p3, orientation);
        Self { t1, t2: None }
    }

    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        println!(
            "generating triangles for points: p1={:?}, p2={:?}, p3={:?}",
            p1, p2, p3
        );
        if let Some(t1) = StandardTriangle::try_new(&p1, &p2, &p3) {
            println!(
                "found simple triangle: p1={:?}, p2={:?}, p3={:?}, orientation: {:?}",
                t1.p1, t1.p2, t1.p3, t1.orientation
            );
            return Self { t1, t2: None };
        }
        let (p1, p2, p3) = prepare_point_order(p1, p2, p3);

        // lets start with the upper triangle:
        let seg_p1_p2 = Point::new(p2.x - p1.x, p2.y - p1.y);
        let swap_base = seg_p1_p2.x < 0;
        let sign_mult = match swap_base {
            true => -1.0,
            false => 1.0,
        };
        let angle_p2_p1_x;
        if seg_p1_p2.x < 0 {
            angle_p2_p1_x = f32::atan(seg_p1_p2.y as f32 / (-1 * seg_p1_p2.x) as f32);
        } else {
            angle_p2_p1_x = f32::atan(seg_p1_p2.y as f32 / seg_p1_p2.x as f32);
        }
        let seg_p3_p2 = Point::new(p2.x - p3.x, p2.y - p3.y);
        let angle_p1_p2_x = (seg_p1_p2.dot(&seg_p3_p2) as f32
            / (seg_p1_p2.magnitude() * seg_p3_p2.magnitude()))
        .acos();

        let angle_p2_x_p1 = PI - angle_p1_p2_x - angle_p2_p1_x;

        let seg_p1_x_x =
            sign_mult * angle_p1_p2_x.sin() * seg_p1_p2.magnitude() / angle_p2_x_p1.sin();

        let p4 = Point::new(p1.x + seg_p1_x_x.round() as i32, p1.y);

        let t1 = match swap_base {
            false => StandardTriangle::new(p1.clone(), p4.clone(), p2, Orientation::Up),
            true => StandardTriangle::new(p4.clone(), p1.clone(), p2, Orientation::Up),
        };

        let t2 = match swap_base {
            false => StandardTriangle::new(p1, p4, p3, Orientation::Down),
            true => StandardTriangle::new(p4, p1, p3, Orientation::Down),
        };

        println!(
            "first: p1={:?}, p2={:?}, p3={:?}, orientation: {:?}",
            t1.p1, t1.p2, t1.p3, t1.orientation
        );
        println!(
            "secon: p1={:?}, p2={:?}, p3={:?}, orientation: {:?}",
            t2.p1, t2.p2, t2.p3, t2.orientation
        );

        Self { t1, t2: Some(t2) }
    }

    pub fn to_triangles(self) -> (StandardTriangle, Option<StandardTriangle>) {
        (self.t1, self.t2)
    }

    pub fn iter_points<'a>(&'a self) -> TriangleIterator<'a> {
        TriangleIterator::new(self)
    }
}

pub struct TriangleIterator<'a> {
    triangle: &'a Triangle,
    cur_iter: StandardTriangleIterator<'a>,
    on_t2: bool,
}

impl<'a> TriangleIterator<'a> {
    pub fn new(triangle: &'a Triangle) -> Self {
        Self {
            triangle,
            cur_iter: triangle.t1.iter_points(),
            on_t2: false,
        }
    }
}

impl<'a> Iterator for TriangleIterator<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_val) = self.cur_iter.next() {
            println!("Point iter found: {:?}", next_val);
            return Some(next_val);
        }
        println!("Point iter finished iterator: on_t2={:?}", self.on_t2);
        if !self.on_t2 {
            self.on_t2 = true;
            if let Some(t2) = &self.triangle.t2 {
                self.cur_iter = t2.iter_points();
                if let Some(next_val) = self.cur_iter.next() {
                    println!("Point iter found: {:?}", next_val);
                    return Some(next_val);
                }
            }
        }
        None
    }
}

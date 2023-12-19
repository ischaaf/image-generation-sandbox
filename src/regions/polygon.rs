use super::{
    bounding_box::BBox, point::Point, region_filter::RegionFilter, triangle::Triangle, ROOT_3,
};

pub struct Polygon {
    triangles: Vec<Triangle>,
    pub bounding_box: BBox,
}

impl Polygon {
    pub fn from_triangles(triangles: Vec<Triangle>) -> Self {
        let mut bot_left = Point::new(i32::MAX, i32::MAX);
        let mut top_right = Point::new(0, 0);
        for triangle in triangles.iter() {
            let bl = triangle.bot_left();
            let tr = triangle.top_right();
            bot_left.x = bot_left.x.min(bl.x);
            bot_left.y = bot_left.y.min(bl.y);
            top_right.x = top_right.x.max(tr.x);
            top_right.y = top_right.y.max(tr.y);
        }
        let bounding_box = BBox::from_bounds(bot_left, top_right);
        Self {
            triangles,
            bounding_box,
        }
    }

    pub fn square(center: Point, size: i32) -> Self {
        let bot_left = Point::new(center.x - size / 2, center.y - size / 2);
        Polygon::rectangle(bot_left, Point { x: size, y: size })
    }

    pub fn rectangle(bot_left: Point, size: Point) -> Self {
        let min_x = bot_left.x;
        let min_y = bot_left.y;
        let max_x = bot_left.x + size.x;
        let max_y = bot_left.y + size.y;
        let bl = bot_left;
        let tl = Point { x: min_x, y: max_y };
        let tr = Point { x: max_x, y: max_y };
        let br = Point { x: max_x, y: min_y };
        Self::from_triangles(vec![
            Triangle::new(bl, tl.clone(), br.clone()),
            Triangle::new(tl, tr, br),
        ])
    }

    pub fn hexagon(center: Point, size: i32) -> Self {
        let size = size + 1;
        let h_unit = size / 2;
        let min_x = center.x - (h_unit as f64 * ROOT_3) as i32;
        let max_x = center.x + (h_unit as f64 * ROOT_3) as i32;
        let min_y = center.y - size;
        let max_y = center.y + size;
        let mid_y_n = center.y - size / 2;
        let mid_y_p = center.y + size / 2;

        let p1 = Point {
            x: min_x,
            y: mid_y_n,
        };
        let p2 = Point {
            x: min_x,
            y: mid_y_p,
        };
        let p3 = Point {
            x: center.x,
            y: max_y,
        };
        let p4 = Point {
            x: max_x,
            y: mid_y_p,
        };
        let p5 = Point {
            x: max_x,
            y: mid_y_n,
        };
        let p6 = Point {
            x: center.x,
            y: min_y,
        };

        let triangles = vec![
            Triangle::new(p1.clone(), p2.clone(), center.clone()),
            Triangle::new(p2, p3.clone(), center.clone()),
            Triangle::new(p3, p4.clone(), center.clone()),
            Triangle::new(p4, p5.clone(), center.clone()),
            Triangle::new(p5, p6.clone(), center.clone()),
            Triangle::new(p6, p1, center),
        ];
        Self::from_triangles(triangles)
    }

    pub fn hexagon_rotated(center: Point, size: i32) -> Self {
        let size = size + 1;
        let v_unit = size / 2;
        let min_y = center.y - (v_unit as f64 * ROOT_3) as i32;
        let max_y = center.y + (v_unit as f64 * ROOT_3) as i32;
        let min_x = center.x - size;
        let max_x = center.x + size;
        let mid_x_n = center.x - size / 2;
        let mid_x_p = center.x + size / 2;

        let p1 = Point {
            y: min_y,
            x: mid_x_n,
        };
        let p2 = Point {
            y: min_y,
            x: mid_x_p,
        };
        let p3 = Point {
            y: center.y,
            x: max_x,
        };
        let p4 = Point {
            y: max_y,
            x: mid_x_p,
        };
        let p5 = Point {
            y: max_y,
            x: mid_x_n,
        };
        let p6 = Point {
            y: center.y,
            x: min_x,
        };

        let triangles = vec![
            Triangle::new(p1.clone(), p2.clone(), center.clone()),
            Triangle::new(p2, p3.clone(), center.clone()),
            Triangle::new(p3, p4.clone(), center.clone()),
            Triangle::new(p4, p5.clone(), center.clone()),
            Triangle::new(p5, p6.clone(), center.clone()),
            Triangle::new(p6, p1, center),
        ];
        Self::from_triangles(triangles)
    }

    pub fn square_center(poly_index: &Point, size: i32, focal_point: &Point) -> Point {
        let x_step = size;
        let y_step = size;
        let min_x = focal_point.x % x_step;
        let min_y = focal_point.y % y_step;

        let result = Point::new(
            min_x + (x_step * poly_index.x),
            min_y + (y_step * poly_index.y),
        );

        result
    }

    pub fn hexagon_center(poly_index: &Point, size: i32, focal_point: &Point) -> Point {
        let x_step = ((size as f64) * ROOT_3).round() as i32;
        let y_step = size + size / 2;
        let min_x = focal_point.x % x_step;
        let min_y = focal_point.y % y_step;

        let result = Point::new(
            min_x + (x_step * poly_index.x) + (x_step / 2 * (poly_index.y % 2)),
            min_y + (y_step * poly_index.y),
        );

        result
    }

    pub fn hexagon_rotated_center(poly_index: &Point, size: i32, focal_point: &Point) -> Point {
        let y_step = ((size as f64) * ROOT_3).round() as i32;
        let x_step = size + size / 2;
        let min_x = focal_point.x % x_step;
        let min_y = focal_point.y % y_step;

        let result = Point::new(
            min_x + (x_step * poly_index.x),
            min_y + (y_step * poly_index.y) + (y_step / 2 * (poly_index.x % 2)),
        );

        result
    }
}

impl RegionFilter for Polygon {
    fn contains(&self, point: &Point) -> bool {
        if !self.bounding_box.contains(point) {
            return false;
        }
        for triangle in self.triangles.iter() {
            if triangle.contains(point) {
                return true;
            }
        }
        return false;
    }
}

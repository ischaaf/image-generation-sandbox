use crate::regions::{Orientation, Point, Polygon, Rectangle, StandardTriangle};

pub const ROOT_3: f64 = 1.732;

pub struct Hexagon {}

impl Hexagon {
    pub fn from_bounds(bounds: &Rectangle) -> Polygon {
        // 4 trianges, 1 rectangle
        let center = bounds.center();
        let size = Point::new(
            ((2 * (center.x - bounds.origin.x)) as f64 / ROOT_3) as i32,
            center.y - bounds.origin.y,
        );
        let min_x = bounds.origin.x;
        let max_x = bounds.extent().x;
        let min_y = bounds.origin.y;
        let max_y = bounds.extent().y;
        let mid_y_n = center.y - size.y / 2;
        let mid_y_p = center.y + size.y / 2;

        let rect = Rectangle::new(
            Point::new(min_x, mid_y_n),
            Point::new(max_x - min_x, mid_y_p - mid_y_n),
        );

        let t_top_left = StandardTriangle::new(
            Point::new(min_x, mid_y_p),
            Point::new(center.x, mid_y_p),
            Point::new(center.x, max_y),
            Orientation::Up,
        );

        let t_top_right = StandardTriangle::new(
            Point::new(center.x, mid_y_p),
            Point::new(max_x, mid_y_p),
            Point::new(center.x, max_y),
            Orientation::Up,
        );

        let t_bot_left = StandardTriangle::new(
            Point::new(min_x, mid_y_n),
            Point::new(center.x, mid_y_n),
            Point::new(center.x, min_y),
            Orientation::Down,
        );
        let t_bot_right = StandardTriangle::new(
            Point::new(center.x, mid_y_n),
            Point::new(max_x, mid_y_n),
            Point::new(center.x, min_y),
            Orientation::Down,
        );

        let triangles = vec![t_top_left, t_top_right, t_bot_left, t_bot_right];
        let rectangles = vec![rect];

        Polygon::from_shapes(triangles, rectangles)
    }

    pub fn tessellate(center: &Point, side_len: i32, bounds: &Rectangle) -> Vec<Polygon> {
        let mut result = Vec::new();
        let step = Self::regular_step(side_len);
        let x_skew = step.x / 2;
        let mut polygon_bounds = Self::regular_bounds(side_len).center_on(center);
        polygon_bounds.origin.x = polygon_bounds.origin.x % step.x;
        polygon_bounds.origin.y = polygon_bounds.origin.y % step.y;

        let row = 0 - (center.y / step.y);
        let mut is_skewed = row % 2 != 0;
        let start_skewed = is_skewed;
        if start_skewed {
            polygon_bounds.origin.x -= x_skew;
        }
        let start_x = polygon_bounds.origin.x;

        'y: loop {
            'x: loop {
                if polygon_bounds.origin.y > bounds.extent().y {
                    break 'y;
                }
                if polygon_bounds.origin.x > bounds.extent().x {
                    break 'x;
                }
                let poly = Self::from_bounds(&polygon_bounds);
                result.push(poly);
                polygon_bounds.origin.x += step.x;
            }
            is_skewed = !is_skewed;
            polygon_bounds.origin.x = start_x;
            if start_skewed && !is_skewed {
                polygon_bounds.origin.x += x_skew;
            } else if !start_skewed && is_skewed {
                polygon_bounds.origin.x += x_skew;
            }
            polygon_bounds.origin.y += step.y;
        }

        result
    }

    pub fn tessellation_center_fn(
        poly_index: &Point,
        normalized_bounds: &Rectangle,
        focal_point: &Point,
    ) -> Rectangle {
        let x_step = normalized_bounds.size.x;
        let x_skew = x_step / 2;

        let y_step = ((normalized_bounds.size.y as f32 * 3.0) / 4.0).round() as i32;
        let tmp_size_x = focal_point.x * 2;
        let tmp_size_y = focal_point.y * 2;

        let min_x = (tmp_size_x % x_step) / 2;
        let min_y = (tmp_size_y % y_step) / 4;

        let row_mult = (focal_point.y / y_step) % 2;
        let x_skew_multiplier = (poly_index.y + row_mult) % 2;

        let origin = Point::new(
            min_x + (x_step * poly_index.x) + x_skew * x_skew_multiplier,
            min_y + (y_step * poly_index.y),
        );
        Rectangle::new(origin, normalized_bounds.size.clone())
    }

    pub fn regular_bounds(side_len: i32) -> Rectangle {
        Rectangle::normal(Point::new(
            (ROOT_3 * side_len as f64).round() as i32,
            2 * side_len,
        ))
    }

    pub fn regular_step(side_len: i32) -> Point {
        let bounds = Self::regular_bounds(side_len);
        Point::new(bounds.size.x, bounds.size.y * 3 / 4)
    }
}

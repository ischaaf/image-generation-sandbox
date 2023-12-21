use crate::regions::{Point, Polygon, Rectangle};

pub struct RectanglePoly {}

impl RectanglePoly {
    pub fn new(bot_left: Point, size: Point) -> Polygon {
        let rect = Rectangle::new(bot_left, size);
        Polygon::from_shapes(vec![], vec![rect])
    }

    pub fn from_bbox(bbox: &Rectangle) -> Polygon {
        let rect = Rectangle::new(bbox.origin.clone(), bbox.size.clone());
        Polygon::from_shapes(vec![], vec![rect])
    }

    pub fn tessellate(
        center: &Point,
        size: &Point,
        bounds: &Rectangle,
        offset: bool,
    ) -> Vec<Polygon> {
        let mut result = Vec::new();
        let step = size.clone();
        let x_skew = size.x / 2;

        let mut polygon_bounds = Rectangle::normal(size.clone()).center_on(center);
        polygon_bounds.origin.x = polygon_bounds.origin.x % step.x;
        polygon_bounds.origin.y = polygon_bounds.origin.y % step.y;

        let row = 0 - (center.y / step.y);
        let mut is_skewed = row % 2 != 0 && offset;
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
                let poly = Self::from_bbox(&polygon_bounds);
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

    pub fn regular_bounds(side_len: i32) -> Rectangle {
        Rectangle::normal(Point::new(side_len, side_len))
    }
}

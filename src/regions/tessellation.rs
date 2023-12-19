use super::{Point, Polygon};

pub struct PolygonTessellation {
    pub polygons: Vec<Polygon>,
}

pub type MakePolygonFn = fn(center: Point, side_len: i32) -> Polygon;
pub type CenterFn = fn(poly_index: &Point, side_len: i32, focal_point: &Point) -> Point;
impl PolygonTessellation {
    pub fn new(
        make_polygon_fn: MakePolygonFn,
        center_fn: CenterFn,
        side_len: i32,
        bounds: &Point,
    ) -> Self {
        let focal_point = Point::new(bounds.x / 2, bounds.y / 2);
        let mut center_index = Point::new(-1, -1);
        let mut center = (center_fn)(&center_index, side_len, &focal_point);

        let mut polygons: Vec<Polygon> = vec![];
        'y: loop {
            'x: loop {
                let poly = (make_polygon_fn)(center.clone(), side_len);
                if poly.bounding_box.origin.y > bounds.y + side_len {
                    break 'y;
                }
                if poly.bounding_box.origin.x > bounds.x {
                    break 'x;
                }
                polygons.push(poly);
                center_index.x += 1;
                center = (center_fn)(&center_index, side_len, &focal_point);
            }
            center_index.y += 1;
            center_index.x = 0;
            center = (center_fn)(&center_index, side_len, &focal_point);
        }

        Self { polygons }
    }
}

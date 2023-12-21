use super::{
    point::Point, Rectangle, RectangleIterator, StandardTriangle, StandardTriangleIterator,
    Triangle,
};

pub struct Polygon {
    triangles: Vec<StandardTriangle>,
    rectangles: Vec<Rectangle>,
    bounding_box: Rectangle,
}

impl Polygon {
    pub fn from_shapes(triangles: Vec<StandardTriangle>, rectangles: Vec<Rectangle>) -> Self {
        println!(
            "Generating polygon with {} triangles and {} rectangles",
            triangles.len(),
            rectangles.len()
        );
        let origin = Point::new(i32::MAX, i32::MAX);
        let extent = Point::new(0, 0);
        let mut bounding_box = Rectangle::from_bounds(origin, extent);
        for tri in triangles.iter() {
            bounding_box.union(&tri.bounds());
        }
        for rect in rectangles.iter() {
            bounding_box.union(&rect.bounds());
        }
        println!("Found boundinb box: {:?}", bounding_box);
        Self {
            triangles,
            rectangles,
            bounding_box,
        }
    }

    pub fn triangle(p1: Point, p2: Point, p3: Point) -> Self {
        let tri = Triangle::new(p1, p2, p3);
        let (t1, maybe_t2) = tri.to_triangles();
        let mut triangles = vec![t1];
        if let Some(t2) = maybe_t2 {
            triangles.push(t2);
        }

        Self::from_shapes(triangles, vec![])
    }

    pub fn iter_points(&self) -> PolygonIterator {
        println!("iterating through polygon");
        let cur_rect_iterator;
        if self.rectangles.len() > 0 {
            cur_rect_iterator = Some(self.rectangles[0].iter_points());
        } else {
            cur_rect_iterator = None;
        }
        let cur_tri_iterator = match self.triangles.len() > 0 {
            true => Some(self.triangles[0].iter_points()),
            false => None,
        };
        PolygonIterator {
            polygon: self,
            cur_rect_index: 0,
            cur_tri_index: 0,
            cur_rect_iterator,
            cur_tri_iterator,
        }
    }

    pub fn bounds(&self) -> Rectangle {
        self.bounding_box.clone()
    }
}

pub struct PolygonIterator<'a> {
    polygon: &'a Polygon,
    cur_rect_index: usize,
    cur_tri_index: usize,
    cur_rect_iterator: Option<RectangleIterator<'a>>,
    cur_tri_iterator: Option<StandardTriangleIterator<'a>>,
}

impl<'a> Iterator for PolygonIterator<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(rect_iter) = &mut self.cur_rect_iterator {
            let rect_result = rect_iter.next();
            if let Some(result) = rect_result {
                println!("Found: {:?} using rectangle", result);
                return Some(result);
            } else {
                loop {
                    self.cur_rect_index += 1;
                    if self.polygon.rectangles.len() > self.cur_rect_index {
                        let mut iter = self.polygon.rectangles[self.cur_rect_index].iter_points();
                        if let Some(result) = iter.next() {
                            self.cur_rect_iterator = Some(iter);
                            println!("Found: {:?} using rectangle", result);
                            return Some(result);
                        }
                    } else {
                        self.cur_rect_iterator = None;
                        break;
                    }
                }
            }
        }
        // ok we finished the rectangles
        if let Some(tri_iter) = &mut self.cur_tri_iterator {
            let tri_result = tri_iter.next();
            if let Some(result) = tri_result {
                println!("Found: {:?} using triangle", result);
                return Some(result);
            } else {
                loop {
                    self.cur_tri_index += 1;
                    if self.polygon.triangles.len() > self.cur_tri_index {
                        let mut iter = self.polygon.triangles[self.cur_tri_index].iter_points();
                        if let Some(result) = iter.next() {
                            self.cur_tri_iterator = Some(iter);
                            println!("Found: {:?} using triangle", result);
                            return Some(result);
                        }
                    } else {
                        self.cur_tri_iterator = None;
                        break;
                    }
                }
            }
        }

        None
    }
}

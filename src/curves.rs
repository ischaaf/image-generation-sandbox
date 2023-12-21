use crate::regions::Point;

pub type CurveFn = Box<dyn Fn(&Point, &Point) -> f32>;

fn apply_ratio(x: f32, d_max: f32, r_start: f32, r_end: f32) -> f32 {
    let ratio = 1.0 - (d_max - x) / d_max;
    (r_end - r_start) * ratio + r_start
}

pub fn constant(val: f32) -> CurveFn {
    Box::new(move |point: &Point, bounds: &Point| val)
}

pub fn linear_x(min: f32, max: f32) -> CurveFn {
    Box::new(move |point: &Point, bounds: &Point| {
        apply_ratio(point.x as f32, bounds.x as f32, min, max)
    })
}

pub fn linear_y(min: f32, max: f32) -> CurveFn {
    Box::new(move |point: &Point, bounds: &Point| {
        apply_ratio(point.y as f32, bounds.y as f32, min, max)
    })
}

pub fn diagonal(min: f32, max: f32) -> CurveFn {
    Box::new(move |point: &Point, bounds: &Point| {
        apply_ratio(
            (point.x + point.y) as f32,
            (bounds.x + bounds.y) as f32,
            min,
            max,
        )
    })
}

pub fn radiate(min: f32, max: f32) -> CurveFn {
    Box::new(move |point: &Point, bounds: &Point| {
        let center = Point::new(bounds.x / 2, bounds.y / 2);
        let distance = point.distance(&center);
        let max_distance = Point::new(0, 0).distance(&center);
        apply_ratio(distance, max_distance, min, max)
    })
}

pub fn radiate_linear(min: f32, max: f32) -> CurveFn {
    Box::new(move |point: &Point, bounds: &Point| {
        let center = Point::new(bounds.x / 2, bounds.y / 2);
        let distance = (center.x.abs_diff(point.x) + center.y.abs_diff(point.y)) as f32;
        let max_distance = (center.x + center.y) as f32;
        apply_ratio(distance, max_distance, min, max)
    })
}

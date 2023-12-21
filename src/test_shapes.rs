use crate::images::Pixel;
use crate::polygons::{Hexagon, RectanglePoly};
use crate::regions::tess::PolygonTessellation;
use crate::regions::{Orientation, Point, Polygon, Rectangle, Region};

use crate::transformers::{
    ColorWaveTF, ColoredTessellationTF, SolidColorPolygon, SolidRectangle, SolidTriangle,
};
use crate::{curves, make_regions};

fn pt(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

pub fn test_octogon() -> Vec<Box<dyn Region>> {
    let center = pt(300, 400);
    let x_step = 100;
    let x_start = center.x - x_step;
    let y_step = 100;
    let y_start = center.y - 2 * y_step;
    let pts = [
        pt(x_start, y_start),
        pt(x_start + 2 * x_step, y_start),
        pt(x_start + 3 * x_step, y_start + 1 * y_step),
        pt(x_start + 3 * x_step, y_start + 3 * y_step),
        pt(x_start + 2 * x_step, y_start + 4 * y_step),
        pt(x_start, y_start + 4 * y_step),
        pt(x_start - 1 * x_step, y_start + 3 * y_step),
        pt(x_start - 1 * x_step, y_start + 1 * y_step),
    ];
    make_regions![
        // center point: 150, 200
        // height: 100
        ColoredTessellationTF::descrete(
            PolygonTessellation::new(vec![
                Polygon::triangle(pts[0].clone(), pts[1].clone(), center.clone()),
                Polygon::triangle(pts[1].clone(), pts[2].clone(), center.clone()),
                Polygon::triangle(pts[2].clone(), pts[3].clone(), center.clone()),
                Polygon::triangle(pts[3].clone(), pts[4].clone(), center.clone()),
                Polygon::triangle(pts[4].clone(), pts[5].clone(), center.clone()),
                Polygon::triangle(pts[5].clone(), pts[6].clone(), center.clone()),
                Polygon::triangle(pts[6].clone(), pts[7].clone(), center.clone()),
                Polygon::triangle(pts[7].clone(), pts[0].clone(), center.clone()),
            ]),
            vec![
                Pixel::hsl(000.0, 1.0, 0.5),
                Pixel::hsl(100.0, 1.0, 0.5),
                Pixel::hsl(200.0, 1.0, 0.5),
                Pixel::hsl(300.0, 1.0, 0.5),
                Pixel::hsl(150.0, 1.0, 0.5),
                Pixel::hsl(350.0, 1.0, 0.5),
                Pixel::hsl(050.0, 1.0, 0.5),
                Pixel::hsl(250.0, 1.0, 0.5),
            ],
        )
    ]
}

pub fn test_gradient() -> Vec<Box<dyn Region>> {
    make_regions![ColorWaveTF::new(
        curves::radiate_linear(0.0, 360.0),
        curves::constant(0.5),
        curves::radiate(0.6, 0.5),
    )]
}

pub fn test_primitives() -> Vec<Box<dyn Region>> {
    make_regions![
        SolidTriangle::new(
            pt(100, 90),
            pt(200, 90),
            pt(150, 190),
            Orientation::Up,
            Pixel::hsl(120.0, 1.0, 0.5),
        ),
        SolidTriangle::new(
            pt(100, 310),
            pt(200, 310),
            pt(150, 210),
            Orientation::Down,
            Pixel::hsl(120.0, 1.0, 0.5),
        ),
        SolidTriangle::new(
            pt(40, 150),
            pt(40, 250),
            pt(140, 200),
            Orientation::Right,
            Pixel::hsl(120.0, 1.0, 0.5),
        ),
        SolidTriangle::new(
            pt(260, 150),
            pt(260, 250),
            pt(160, 200),
            Orientation::Left,
            Pixel::hsl(120.0, 1.0, 0.5),
        ),
        // right triangles
        // center: 150, 450
        SolidTriangle::new(
            pt(40, 340),
            pt(140, 340),
            pt(40, 440),
            Orientation::Up,
            Pixel::hsl(180.0, 1.0, 0.5),
        ),
        SolidTriangle::new(
            pt(160, 560),
            pt(260, 560),
            pt(260, 460),
            Orientation::Down,
            Pixel::hsl(180.0, 1.0, 0.5),
        ),
        SolidTriangle::new(
            pt(40, 460),
            pt(40, 560),
            pt(140, 560),
            Orientation::Right,
            Pixel::hsl(180.0, 1.0, 0.5),
        ),
        SolidTriangle::new(
            pt(260, 340),
            pt(260, 440),
            pt(160, 340),
            Orientation::Left,
            Pixel::hsl(180.0, 1.0, 0.5),
        ),
        // square
        SolidRectangle::new(pt(140, 190), pt(20, 20), Pixel::hsl(180.0, 1.0, 0.5),),
        SolidRectangle::new(pt(100, 400), pt(100, 100), Pixel::hsl(120.0, 1.0, 0.5),),
        SolidColorPolygon::new(
            Hexagon::from_bounds(&Rectangle::new(pt(400, 400), pt(100, 100))),
            Pixel::hsl(200.0, 1.0, 0.5),
        )
    ]
}

pub fn test_tessellation() -> Vec<Box<dyn Region>> {
    make_regions![
        ColoredTessellationTF::new(
            PolygonTessellation::new(Hexagon::tessellate(
                &pt(300, 400),
                30,
                &Rectangle::new(pt(100, 100), pt(400, 600)),
            )),
            vec![
                Pixel::hsl(0.0, 1.0, 0.5),
                Pixel::hsl(120.0, 1.0, 0.5),
                Pixel::hsl(240.0, 1.0, 0.5),
            ],
        ),
        ColoredTessellationTF::new(
            PolygonTessellation::new(RectanglePoly::tessellate(
                &pt(300, 400),
                &pt(30, 30),
                &Rectangle::new(pt(200, 200), pt(200, 400)),
                false,
            )),
            vec![
                Pixel::hsl(0.0, 1.0, 0.8),
                Pixel::hsl(120.0, 1.0, 0.8),
                Pixel::hsl(240.0, 1.0, 0.8),
            ],
        )
    ]
}

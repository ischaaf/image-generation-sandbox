use regions::{
    tess::PolygonTessellation, Orientation, Point, Polygon, Rectangle, Region, Triangle,
};
use transformers::{
    ColorWaveTF, ColoredTessellationTF, SolidColorPolygon, SolidRectangle, SolidTriangle,
};

use polygons::{Hexagon, RectanglePoly};

use crate::images::{Image, Pixel};

const BASE_PATH: &str = "/mnt/c/Users/isaac/Pictures/patterns/";

mod curves;
mod image_writer;
mod images;
mod polygons;
mod regions;
mod transformers;

macro_rules! make_regions {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<Box<dyn Region>> = Vec::new();
            $(
                temp_vec.push(Box::new($x));
            )*
            temp_vec
        }
    };
}

fn pt(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

fn main() {
    let bounds = Point::new(600, 800);
    let mut image = Image::new(bounds.x, bounds.y);
    // let mut tfs = make_tfs![
    //     (
    //         ColorWaveTF::new(
    //             curves::radiate_linear(0.0, 360.0),
    //             curves::constant(0.5),
    //             curves::radiate(0.6, 0.5),
    //         ),
    //         NoF {}
    //     ),
    //     (
    //         BlendedTessellationTF::new(
    //             Polygon::hexagon_rotated,
    //             Polygon::hexagon_rotated_center,
    //             5,
    //             &image.size.clone(),
    //         ),
    //         NoF {}
    //     ) // (Blend {}, NoF {})
    // ];

    let (t1, maybe_t2) = Triangle::new(pt(200, 225), pt(175, 250), pt(150, 200)).to_triangles();
    let t2 = maybe_t2.unwrap();

    let center = pt(150, 200);
    let x_step = 51;
    let x_start = center.x - x_step;
    let y_step = 51;
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

    let mut regions: Vec<Box<dyn Region>> = make_regions![
        // center point: 150, 200
        // height: 100
        SolidRectangle::new(
            Point::new(100, 100),
            Point::new(400, 600),
            Pixel::hsla(0.0, 0.0, 0.0, 0xff),
        ),
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
                Pixel::hsla(000.0, 1.0, 0.5, 0xff),
                Pixel::hsla(100.0, 1.0, 0.5, 0xff),
                Pixel::hsla(200.0, 1.0, 0.5, 0xff),
                Pixel::hsla(300.0, 1.0, 0.5, 0xff),
                Pixel::hsla(150.0, 1.0, 0.5, 0xff),
                Pixel::hsla(350.0, 1.0, 0.5, 0xff),
                Pixel::hsla(050.0, 1.0, 0.5, 0xff),
                Pixel::hsla(250.0, 1.0, 0.5, 0xff),
            ],
        ),
        ColoredTessellationTF::descrete(
            PolygonTessellation::new(vec![
                Polygon::from_shapes(vec![t1], vec![]),
                Polygon::from_shapes(vec![t2], vec![]),
            ]),
            vec![
                Pixel::hsla(300.0, 1.0, 0.5, 0xff),
                Pixel::hsla(150.0, 1.0, 0.5, 0xff),
            ],
        ),
        SolidTriangle::new_any(
            Point::new(200, 200),
            Point::new(400, 250),
            Point::new(400, 400),
            Pixel::hsla(120.0, 1.0, 0.5, 0xff),
        ),
        SolidRectangle::new(
            Point::new(100, 200),
            Point::new(400, 200),
            Pixel::hsla(360.0, 0.9, 0.5, 0xff),
        ),
        SolidRectangle::new(
            Point::new(200, 100),
            Point::new(200, 400),
            Pixel::hsla(360.0, 0.5, 0.5, 0xff),
        ),
        SolidTriangle::new(
            Point::new(200, 200),
            Point::new(400, 200),
            Point::new(400, 400),
            Orientation::Up,
            Pixel::hsla(120.0, 1.0, 0.5, 0xff),
        ),
        SolidTriangle::new(
            Point::new(200, 400),
            Point::new(400, 400),
            Point::new(200, 200),
            Orientation::Down,
            Pixel::hsla(240.0, 1.0, 0.5, 0xff),
        ),
        ColoredTessellationTF::new(
            PolygonTessellation::new(Hexagon::tessellate(
                &Point::new(300, 400),
                30,
                &Rectangle::new(Point::new(100, 100), Point::new(400, 600)),
            )),
            vec![
                Pixel::hsla(0.0, 1.0, 0.5, 0xff),
                Pixel::hsla(120.0, 1.0, 0.5, 0xff),
                Pixel::hsla(240.0, 1.0, 0.5, 0xff),
            ],
        ),
        ColoredTessellationTF::new(
            PolygonTessellation::new(RectanglePoly::tessellate(
                &Point::new(300, 400),
                &Point::new(30, 30),
                &Rectangle::new(Point::new(200, 200), Point::new(200, 400)),
                false,
            )),
            vec![
                Pixel::hsla(0.0, 1.0, 0.8, 0xff),
                Pixel::hsla(120.0, 1.0, 0.8, 0xff),
                Pixel::hsla(240.0, 1.0, 0.8, 0xff),
            ],
        ),
        ColorWaveTF::new(
            curves::radiate_linear(0.0, 360.0),
            curves::constant(0.5),
            curves::radiate(0.6, 0.5),
        ),
        SolidRectangle::new(
            Point::new(298, 398),
            Point::new(4, 4),
            Pixel::hsla(0.0, 0.0, 0.0, 0xff),
        ),
        SolidTriangle::new(
            Point::new(100, 90),
            Point::new(200, 90),
            Point::new(150, 190),
            Orientation::Up,
            Pixel::hsla(120.0, 1.0, 0.5, 0xff),
        ),
        SolidTriangle::new(
            Point::new(100, 310),
            Point::new(200, 310),
            Point::new(150, 210),
            Orientation::Down,
            Pixel::hsla(120.0, 1.0, 0.5, 0xff),
        ),
        SolidTriangle::new(
            Point::new(40, 150),
            Point::new(40, 250),
            Point::new(140, 200),
            Orientation::Right,
            Pixel::hsla(120.0, 1.0, 0.5, 0xff),
        ),
        SolidTriangle::new(
            Point::new(260, 150),
            Point::new(260, 250),
            Point::new(160, 200),
            Orientation::Left,
            Pixel::hsla(120.0, 1.0, 0.5, 0xff),
        ),
        // right triangles
        // center: 150, 450
        SolidTriangle::new(
            Point::new(40, 340),
            Point::new(140, 340),
            Point::new(40, 440),
            Orientation::Up,
            Pixel::hsla(180.0, 1.0, 0.5, 0xff),
        ),
        SolidTriangle::new(
            Point::new(160, 560),
            Point::new(260, 560),
            Point::new(260, 460),
            Orientation::Down,
            Pixel::hsla(180.0, 1.0, 0.5, 0xff),
        ),
        SolidTriangle::new(
            Point::new(40, 460),
            Point::new(40, 560),
            Point::new(140, 560),
            Orientation::Right,
            Pixel::hsla(180.0, 1.0, 0.5, 0xff),
        ),
        SolidTriangle::new(
            Point::new(260, 340),
            Point::new(260, 440),
            Point::new(160, 340),
            Orientation::Left,
            Pixel::hsla(180.0, 1.0, 0.5, 0xff),
        ),
        // square
        SolidRectangle::new(
            Point::new(140, 190),
            Point::new(20, 20),
            Pixel::hsla(180.0, 1.0, 0.5, 0xff),
        ),
        SolidRectangle::new(
            Point::new(100, 400),
            Point::new(100, 100),
            Pixel::hsla(120.0, 1.0, 0.5, 0xff),
        ),
        SolidColorPolygon::new(
            Hexagon::from_bounds(&Rectangle::new(Point::new(400, 400), Point::new(100, 100))),
            Pixel::hsla(200.0, 1.0, 0.5, 0xff),
        )
    ];

    for (i, region) in regions.iter_mut().enumerate() {
        image.apply_region(region.as_mut());
        image.write(format!("{}/{:02}_output.png", BASE_PATH, i).as_str());
    }
}

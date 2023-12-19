use regions::{NoF, Point, Polygon, RegionFilter};
use transformers::{BlendedTessellationTF, ColorWaveTF, ImageTransformer, SolidColorTransformer};

use crate::images::{Image, Pixel};

const BASE_PATH: &str = "/mnt/c/Users/isaac/Pictures/patterns/";

mod curves;
mod image_writer;
mod images;
mod regions;
mod transformers;

const fn rgb(r: u8, g: u8, b: u8) -> Pixel {
    Pixel::rgb(r, g, b)
}

const RED: Pixel = rgb(0xff, 0, 0);
const GREEN: Pixel = rgb(0, 0xff, 0);
const BLUE: Pixel = rgb(0, 0, 0xff);

fn main() {
    let [c1, c2, c3, c4] = RED.luminate(0.3).set_alpha(0x44).tetradic();
    let mut image = Image::new(600, 800);
    let mut tfs: Vec<(Box<dyn ImageTransformer>, Box<dyn RegionFilter>)> = vec![
        (
            // Box::new(SolidColorTransformer::new(Pixel::rgb(0x00, 0x00, 0x00))),
            Box::new(ColorWaveTF::new(
                curves::linear_x(120.0, 300.0),
                curves::constant(0.5),
                curves::constant(0.5),
            )),
            Box::new(NoF {}),
        ),
        // (
        //     Box::new(BlendedTessellationTF::new(
        //         Polygon::hexagon,
        //         Polygon::hexagon_center,
        //         40,
        //         &image.size.clone(),
        //     )),
        //     Box::new(NoF {}),
        // ),
        (
            Box::new(BlendedTessellationTF::new(
                Polygon::hexagon_rotated,
                Polygon::hexagon_rotated_center,
                40,
                &image.size.clone(),
            )),
            Box::new(NoF {}),
        ),
        (
            Box::new(SolidColorTransformer::new(Pixel::rgb(0, 0, 0))),
            Box::new(Polygon::square(Point::new(299, 399), 2)),
        ),
    ];

    for (i, (tf, r)) in tfs.iter_mut().enumerate() {
        image.transform_region(tf.as_mut(), r.as_ref());
        image.write(format!("{}/{:02}_output.png", BASE_PATH, i).as_str());
    }
}

fn test_pixel(r: u8, g: u8, b: u8, expected_h: f32, expected_s: f32, expected_l: f32) {
    let pixel = Pixel::rgba(r, g, b, 1);
    let (h, s, l, a) = pixel.to_hsla();
    let recreated = Pixel::hsla(h, s, l, a);

    println!(
        "found   : rgb({}, {}, {}) -> hsl({}, {:.2}, {:.2}) -> rgb({}, {}, {})",
        pixel.r, pixel.g, pixel.b, h, s, l, recreated.r, recreated.g, recreated.b
    );
    println!(
        "expected: rgb({}, {}, {}) -> hsl({}, {}, {}) -> rgb({}, {}, {})",
        r, g, b, expected_h, expected_s, expected_l, r, g, b
    );
    println!();
}

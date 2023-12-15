use colors::{transformers::{CheckerboardTF, SolidColorTransformer, ImageTransformer}, regions::{RegionFilter, NoF, MutliRegion, TriangleF, Polygon}};

use crate::colors::{Image, Pixel};

const BASE_PATH: &str = "/mnt/c/Users/isaac/Pictures/patterns/";

mod image_writer;
mod colors;

const fn rgb(r: u8, g: u8, b: u8) -> Pixel {
    Pixel::rgb(r, g, b)
}

const RED: Pixel = rgb(0xff, 0, 0);
const GREEN: Pixel = rgb(0, 0xff, 0);
const BLUE: Pixel = rgb(0, 0, 0xff);

fn main() {
    let mut image = Image::new(600, 800);
    let tfs: Vec<(Box<dyn ImageTransformer>, Box<dyn RegionFilter>)> = vec![
        (Box::new(SolidColorTransformer::new(Pixel::rgb(0xff, 0, 0x33))), Box::new(NoF{})),
        (Box::new(CheckerboardTF::new(20, RED, BLUE)), Box::new(
                MutliRegion::new(vec![
                    Box::new(Polygon::square((0, 0), 99)),
                    Box::new(Polygon::square((0, 700), 99)),
                    Box::new(Polygon::square((500, 0), 99)),
                    Box::new(Polygon::square((500, 700), 99)),
                    // Box::new(TriangleF::new((200, 200), (200, 500), (300, 100))),
                ])
            )
        ),
    ];

    for (i, (tf, r)) in tfs.iter().enumerate() {
        image.transform_region(tf.as_ref(), r.as_ref());
        println!("pixel val: {:?}, with u32: {:08x}", image.data[0], image.data[0].to_combined());
        image.write(format!("{}/{:02}_output.png", BASE_PATH, i).as_str());
    }


}

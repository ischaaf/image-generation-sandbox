use crate::images::Image;
use regions::{Point, Region};

const BASE_PATH: &str = "/mnt/c/Users/isaac/Pictures/patterns/";

mod curves;
mod image_writer;
mod images;
mod macros;
mod polygons;
mod regions;
mod test_shapes;
mod transformers;

fn main() {
    apply_regions("01_TestOctogon", test_shapes::test_octogon());
    // apply_regions("02_TestTessellation", test_shapes::test_tessellation());
    // apply_regions("03_TestPrimitives", test_shapes::test_primitives());
    // apply_regions("04_TestGradient", test_shapes::test_gradient());
}

fn apply_regions(name: &str, rns: Vec<Box<dyn Region>>) {
    let bounds = Point::new(600, 800);
    let mut image = Image::new(bounds.x, bounds.y);
    let mut rns = rns;
    for (i, region) in rns.iter_mut().enumerate() {
        image.apply_region(region.as_mut());
        image.write(format!("{}/{}_{:02}.png", BASE_PATH, name, i).as_str());
    }
}

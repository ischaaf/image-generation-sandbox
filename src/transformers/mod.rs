use crate::{
    images::{Image, Pixel},
    regions::Point,
};

pub trait ImageTransformer {
    fn transform_pixel(&mut self, point: &Point, value: &Pixel, image: &Image) -> Pixel;
}

mod blended_tessellation;
mod checkerboard;
mod colored_tessellation;
mod gradient;
mod solid_color;

pub use blended_tessellation::BlendedTessellationTF;
pub use checkerboard::CheckerboardTF;
pub use colored_tessellation::ColoredTessellationTF;
pub use gradient::ColorWaveTF;
pub use solid_color::SolidColorTransformer;

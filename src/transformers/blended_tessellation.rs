use crate::{
    images::{Image, Pixel},
    regions::{tess, Point, Region},
};

pub struct BlendedTessellationTF {
    tessellation: tess::PolygonTessellation,
}

impl BlendedTessellationTF {
    pub fn new(tessellation: tess::PolygonTessellation) -> Self {
        Self { tessellation }
    }
}

impl Region for BlendedTessellationTF {
    fn get_mutations(&self, image: &Image, mutations: &mut Vec<(Point, Pixel)>) {
        for polygon in self.tessellation.polygons.iter() {
            let mut pixels_to_blend = vec![];
            for point in polygon.iter_points() {
                if image.contains(&point) {
                    pixels_to_blend.push(image.get_pixel(&point));
                }
            }

            if pixels_to_blend.len() == 0 {
                continue;
            }

            let mut alpha: u8 = (255 / pixels_to_blend.len()) as u8;
            if alpha == 0 {
                alpha = 1;
            }
            let alpha = alpha;

            let mut blended_pixel = pixels_to_blend[0].clone().set_alpha(alpha);
            for pixel_to_blind in pixels_to_blend.iter().skip(1) {
                blended_pixel = blended_pixel.blend(&pixel_to_blind.clone().set_alpha(alpha));
            }

            for point in polygon.iter_points() {
                if image.contains(&point) {
                    mutations.push((point, blended_pixel.clone()));
                }
            }
        }
    }
}

mod blended_tessellation;
mod colored_tessellation;
mod gradient;
mod solid_color;
mod solid_triangle;

pub use blended_tessellation::BlendedTessellationTF;
pub use colored_tessellation::ColoredTessellationTF;
pub use gradient::ColorWaveTF;
pub use solid_color::SolidColorPolygon;
pub use solid_triangle::{SolidRectangle, SolidTriangle};

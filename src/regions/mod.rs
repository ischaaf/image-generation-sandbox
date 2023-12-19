mod bounding_box;
mod multi_region;
mod point;
mod polygon;
mod region_filter;
mod tessellation;
mod triangle;

pub const ROOT_3: f64 = 1.732;

pub use bounding_box::BBox;
pub use multi_region::MutliRegion;
pub use point::Point;
pub use polygon::Polygon;
pub use region_filter::{NoF, RegionFilter};
pub use triangle::Triangle;

pub mod tess {
    pub use super::tessellation::CenterFn;
    pub use super::tessellation::MakePolygonFn;
    pub use super::tessellation::PolygonTessellation;
}

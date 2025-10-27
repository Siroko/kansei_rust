// Geometries module
pub mod geometry;
pub mod box_geometry;
pub mod plane_geometry;

pub use geometry::{Geometry, Vertex};
pub use box_geometry::BoxGeometry;
pub use plane_geometry::PlaneGeometry;


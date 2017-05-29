pub use self::bounding_box3::BoundingBox3;
pub use self::triangle3::Triangle3;
pub use self::traits::*;

mod bounding_box3;
mod traits;
mod triangle3;
mod triangular_mesh;

#[cfg(test)]
mod test;

use common::{constants, utils};
use geometry::{HasSurfaceArea, HasVolume, HasBoundingBox3, BoundingBox3};
use math::Vector3;
use std::cmp::{PartialEq};
use std::fmt;

/// Definitions and operations of a triangle in 3D space, storing its three
/// vertices in counter-clockwise order (with up direction pointing to the
/// exterior of the shape).
///
/// The triangle is immutable after initialized. To modify or update the
/// triangle, copy and initialize a new one.
#[derive(Debug, Copy, Clone)]
pub struct Triangle3 {
  vertices: [Vector3; 3],
}

impl Triangle3 {
  /// Returns the vertice at a given index. Vertices are stored in the same
  /// order as they're passed in when initilized.
  pub fn v(&self, ind: usize) -> Vector3 {
    assert!(ind < 3, "Index out of bound!");
    self.vertices[ind]
  }

  /// Returns the edge at a given index. Edges are indexed as follows:
  ///
  /// - Edge #0: v0 -> v1;
  /// - Edge #1: v1 -> v2;
  /// - Edge #2: v2 -> v0.
  pub fn e(&self, ind: usize) -> Vector3 {
    assert!(ind < 3, "Index out of bound!");
    self.vertices[(ind + 1) % 3] - self.vertices[ind]
  }

  pub fn normal(&self) -> Vector3 {
    let n : Vector3 = self.e(0) ^ self.e(1);
    assert!(n.len2() > constants::EPSILON_TINY);
    n.normalize()
  }

  pub fn new(v1: Vector3, v2: Vector3, v3: Vector3) -> Self {
    Triangle3 {
      vertices: [v1, v2, v3],
    }
  }

  pub fn new_from_array(v: &[Vector3; 3]) -> Self {
    Triangle3 {
      vertices: [v[0], v[1], v[2]],
    }
  }
}

/// Overriding == and != to allow comparison within error bounds.
impl PartialEq for Triangle3 {
  fn eq(&self, rhs: &Self) -> bool {
    self.vertices[0] == rhs.vertices[0]
        && self.vertices[1] == rhs.vertices[1]
        && self.vertices[2] == rhs.vertices[2]
  }

  fn ne(&self, rhs: &Self) -> bool {
    !self.eq(rhs)
  }
}

impl HasSurfaceArea for Triangle3 {
  fn surface_area(&self) -> f64 {
    (self.e(0) ^ self.e(1)).len() / 2.0
  }
}

impl HasVolume for Triangle3 {
  fn volume(&self) -> f64 {
    0.0
  }
}

impl HasBoundingBox3 for Triangle3 {
  fn bounding_box3(&self) -> BoundingBox3 {
    let v = self.vertices;
    BoundingBox3::new_from_nums(
      utils::min(v[0].x(), v[1].x(), v[2].x()),
      utils::min(v[0].y(), v[1].y(), v[2].y()),
      utils::min(v[0].z(), v[1].z(), v[2].z()),
      utils::max(v[0].x(), v[1].x(), v[2].x()),
      utils::max(v[0].y(), v[1].y(), v[2].y()),
      utils::max(v[0].z(), v[1].z(), v[2].z()))
  }
}

/// Print the triangle as:
/// Triangle3(v1, v2, v3)
impl fmt::Display for Triangle3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Triangle3({}, {}, {})",
        &self.vertices[0], &self.vertices[1], &self.vertices[2])
  }
}

use std::fmt;
use std::cmp::PartialEq;
use math::Vector3;
use geometry::{HasSurfaceArea, HasVolume};

// Definitions and operations of a 3-d bounding box, with all edges parallel
// with axis. Upper-left-front corner (min corner) and bottom-right-rear corner
// are stored to represent the box.
#[derive(Debug, Copy, Clone)]
pub struct BoundingBox3 {
  min_corner: Vector3,
  max_corner: Vector3
}

impl BoundingBox3 {
  pub fn min_corner(&self) -> Vector3 {
    self.min_corner
  }

  pub fn max_corner(&self) -> Vector3 {
    self.max_corner
  }

  pub fn min_x(&self) -> f64 {
    self.min_corner.x()
  }

  pub fn min_y(&self) -> f64 {
    self.min_corner.y()
  }

  pub fn min_z(&self) -> f64 {
    self.min_corner.z()
  }

  pub fn max_x(&self) -> f64 {
    self.max_corner.x()
  }

  pub fn max_y(&self) -> f64 {
    self.max_corner.x()
  }

  pub fn max_z(&self) -> f64 {
    self.max_corner.z()
  }

  pub fn len_x(&self) -> f64 {
    self.max_corner.x() - self.min_corner.x()
  }

  pub fn len_y(&self) -> f64 {
    self.max_corner.y() - self.min_corner.y()
  }

  pub fn len_z(&self) -> f64 {
    self.max_corner.z() - self.min_corner.z()
  }

  pub fn size(&self) -> Vector3 {
    self.max_corner - self.min_corner
  }

  // Moves this bounding box along a specified direction.
  pub fn move_self(&mut self, dir: Vector3) {
    self.min_corner += dir;
    self.max_corner += dir;
  }

  // Returns a new bounding box by moving this one along a specified direction.
  pub fn move_by(self, dir: Vector3) -> Self {
    let mut b = self;
    b.move_self(dir);
    b
  }

  pub fn new(min_corner: Vector3, max_corner: Vector3) -> Self {
    assert!(max_corner.x() >= min_corner.x());
    assert!(max_corner.y() >= min_corner.y());
    assert!(max_corner.z() >= min_corner.z());
    Self {
      min_corner: min_corner,
      max_corner: max_corner,
    }
  }

  // Returns a new empty bounding box with min and max corners be at (0, 0, 0).
  pub fn zero() -> Self {
    Self::new(Vector3::zero(), Vector3::zero())
  }

  pub fn new_from_nums(min_x: f64, min_y: f64, min_z: f64,
      max_x: f64, max_y: f64, max_z: f64) -> Self {
    Self::new(
        Vector3::new(min_x, min_y, min_z),
        Vector3::new(max_x, max_y, max_z))
  }
}

/// Overriding == and != to allow comparison within error bounds.
impl PartialEq for BoundingBox3 {
  fn eq(&self, rhs: &Self) -> bool {
    self.min_corner == rhs.min_corner && self.max_corner == rhs.max_corner
  }

  fn ne(&self, rhs: &Self) -> bool {
    !self.eq(rhs)
  }
}

impl HasSurfaceArea for BoundingBox3 {
  fn surface_area(&self) -> f64 {
    2.0 * ( self.len_x() * self.len_y()
        + self.len_y() * self.len_z()
        + self.len_z() * self.len_x())
  }
}

impl HasVolume for BoundingBox3 {
  fn volume(&self) -> f64 {
    self.len_x() * self.len_y() * self.len_z()
  }
}

/// Printing the bounding box as
/// (min_x, min_y, min_z) - (max_x, max_y, max_z).
impl fmt::Display for BoundingBox3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} - {}", &self.min_corner, &self.max_corner)
  }
}

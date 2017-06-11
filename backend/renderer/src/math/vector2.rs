use common::constants;
use std::fmt;
use std::ops::{
  Add, AddAssign, Sub, SubAssign, Neg,
  Div, DivAssign, Mul, MulAssign,
  Index, IndexMut,
};
use std::cmp::PartialEq;

/// Definition and operations of a 2-dimentional double-precision vector.
/// This class is usually used to store UV mapping coordinates. Hence its
/// components are u and v, rather than x, y and z. Operations and properties
/// are also limited to mapping purposes.
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
  v: [f64; 2],
}

impl Vector2 {
  /// Returns u component of the vector.
  pub fn u(&self) -> f64 {
    self.v[0]
  }

  /// Returns v component of the vector.
  pub fn v(&self) -> f64 {
    self.v[1]
  }

  /// Returns the norm-2 length of the vector.
  pub fn len(&self) -> f64 {
    self.len2().sqrt()
  }

  /// Returns the square of norm-2 length of the vector. This method is
  /// usually used when you just need the square of the length, but do not
  /// want to involve a redundant sqrt calculation.
  pub fn len2(&self) -> f64 {
    self.v[0] * self.v[0] + self.v[1] * self.v[1]
  }

  /// Converts the vector into an array of length 2.
  pub fn to_array(&self) -> [f64; 2] {
    [self.v[0], self.v[1]]
  }

  /// Constructor from two components.
  pub fn new(u: f64, v: f64) -> Self {
    Self {
      v: [u, v],
    }
  }

  /// Constructs a vector from an array of length 3.
  pub fn new_from_array(arr: &[f64; 2]) -> Self {
    Self {
      v: [arr[0], arr[1]],
    }
  }

  /// Constructs a zero vector, i.e. (0, 0, 0).
  pub fn zero() -> Self {
    Self::new(0.0, 0.0)
  }
}

/// Printing the vector as (x, y, z).
impl fmt::Display for Vector2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", &self.v[0], &self.v[1])
  }
}

/// Overriding += for vector addition.
impl AddAssign for Vector2 {
  fn add_assign(&mut self, rhs: Self) {
    self.v[0] += rhs.v[0];
    self.v[1] += rhs.v[1];
  }
}

/// Overriding + for vector addition.
impl Add for Vector2 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    let mut v = self;
    v += rhs;
    v
  }
}

/// Overriding -= for vector subtraction.
impl SubAssign for Vector2 {
  fn sub_assign(&mut self, rhs: Self) {
    self.v[0] -= rhs.v[0];
    self.v[1] -= rhs.v[1];
  }
}

/// Overriding - for vector subtraction.
impl Sub for Vector2 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    let mut v = self;
    v -= rhs;
    v
  }
}

/// Overriding - for vector negation.
impl Neg for Vector2 {
  type Output = Self;
  fn neg(self) -> Self {
    Vector2::zero() - self
  }
}

/// Overriding *= for vector multiplication with a numerical.
impl MulAssign<f64> for Vector2 {
  fn mul_assign(&mut self, rhs: f64) {
    self.v[0] *= rhs;
    self.v[1] *= rhs;
  }
}

/// Overriding * for vector multiplication with a numerical.
impl Mul<f64> for Vector2 {
  type Output = Self;
  fn mul(self, rhs: f64) -> Self {
    let mut v = self;
    v *= rhs;
    v
  }
}

/// Overriding /= for vector division with a numerical.
impl DivAssign<f64> for Vector2 {
  fn div_assign(&mut self, rhs: f64) {
    assert!(rhs.abs() > constants::EPSILON_TINY, "Division by zero!");
    *self *= 1.0 / rhs;
  }
}

/// Overriding / for vector division with a numerical.
impl Div<f64> for Vector2 {
  type Output = Self;
  fn div(self, rhs: f64) -> Self {
    let mut v = self;
    v /= rhs;
    v
  }
}

/// Overriding [] for immutable indexing.
impl Index<usize> for Vector2 {
  type Output = f64;
  fn index(&self, ind: usize) -> &f64 {
    assert!(ind < 2, "Index out of bound!");
    &self.v[ind]
  }
}

/// Overriding [] for mutable indexing.
impl IndexMut<usize> for Vector2 {
  fn index_mut(&mut self, ind: usize) -> &mut f64 {
    assert!(ind < 2, "Index out of bound!");
    &mut self.v[ind]
  }
}

/// Overriding == and != to allow comparison within error bounds.
impl PartialEq for Vector2 {
  fn eq(&self, rhs: &Self) -> bool {
    (self.v[0] - rhs.v[0]).abs() < constants::EPSILON
        && (self.v[1] - rhs.v[1]).abs() < constants::EPSILON
  }

  fn ne(&self, rhs: &Self) -> bool {
    !self.eq(rhs)
  }
}

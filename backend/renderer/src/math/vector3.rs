use common::constants;
use std::fmt;
use std::ops::{
  Add, AddAssign, Sub, SubAssign, Neg,
  Div, DivAssign, Mul, MulAssign,
  Index, IndexMut, BitXor, BitXorAssign,
};
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
  v: [f64; 3],
}

impl Vector3 {
  /// Properties.
  pub fn x(&self) -> f64 {
    self.v[0]
  }

  pub fn y(&self) -> f64 {
    self.v[1]
  }

  pub fn z(&self) -> f64 {
    self.v[2]
  }

  pub fn len(&self) -> f64 {
    self.len2().sqrt()
  }

  pub fn len2(&self) -> f64 {
    (*self) * (*self)
  }

  pub fn to_array(&self) -> [f64; 3] {
    [self.v[0], self.v[1], self.v[2]]
  }

  pub fn normalize(&self) -> Self {
    let mut v = self.clone();
    v.normalize_self();
    v
  }

  pub fn normalize_self(&mut self) -> f64 {
    let mut len = self.len2();
    if len < constants::EPSILON_TINY {
      return 0.0;
    }
    len = len.sqrt();
    *self /= len;
    len
  }

  /// Constructors and factories
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Self {
      v: [x, y, z],
    }
  }

  pub fn new_from_array(arr: &[f64; 3]) -> Self {
    Self {
      v: [arr[0], arr[1], arr[2]],
    }
  }

  pub fn identity(s: f64) -> Self {
    Self::new(s, s, s)
  }

  pub fn zero() -> Self {
    Self::identity(0.0)
  }

  pub fn one() -> Self {
    Self::identity(1.0)
  }

  pub fn x_unit() -> Self {
    Self::new(1.0, 0.0, 0.0)
  }

  pub fn y_unit() -> Self {
    Self::new(0.0, 1.0, 0.0)
  }

  pub fn z_unit() -> Self {
    Self::new(0.0, 0.0, 1.0)
  }
}

impl fmt::Display for Vector3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {})", self.v[0], self.v[1], self.v[2])
  }
}

/// Overriding += for vector addition.
impl AddAssign for Vector3 {
  fn add_assign(&mut self, rhs: Self) {
    self.v[0] += rhs.v[0];
    self.v[1] += rhs.v[1];
    self.v[2] += rhs.v[2];
  }
}

/// Overriding + for vector addition.
impl Add for Vector3 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    let mut v = self;
    v += rhs;
    v
  }
}

/// Overriding -= for vector subtraction.
impl SubAssign for Vector3 {
  fn sub_assign(&mut self, rhs: Self) {
    self.v[0] -= rhs.v[0];
    self.v[1] -= rhs.v[1];
    self.v[2] -= rhs.v[2];
  }
}

/// Overriding - for vector subtraction.
impl Sub for Vector3 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    let mut v = self;
    v -= rhs;
    v
  }
}

/// Overriding - for vector negation.
impl Neg for Vector3 {
  type Output = Self;
  fn neg(self) -> Self {
    Vector3::zero() - self
  }
}

/// Overriding *= for vector multiplication with a numerical.
impl MulAssign<f64> for Vector3 {
  fn mul_assign(&mut self, rhs: f64) {
    self.v[0] *= rhs;
    self.v[1] *= rhs;
    self.v[2] *= rhs;
  }
}

/// Overriding * for vector multiplication with a numerical.
impl Mul<f64> for Vector3 {
  type Output = Self;
  fn mul(self, rhs: f64) -> Self {
    let mut v = self;
    v *= rhs;
    v
  }
}

/// Overriding * for vector inner product.
impl Mul for Vector3 {
  type Output = f64;
  fn mul(self, rhs: Self) -> f64 {
    self.v[0] * rhs.v[0] + self.v[1] * rhs.v[1] + self.v[2] * rhs.v[2]
  }
}

/// Overriding /= for vector division with a numerical.
impl DivAssign<f64> for Vector3 {
  fn div_assign(&mut self, rhs: f64) {
    assert!(rhs.abs() > constants::EPSILON_TINY, "Division by zero!");
    *self *= 1.0 / rhs;
  }
}

/// Overriding / for vector division with a numerical.
impl Div<f64> for Vector3 {
  type Output = Self;
  fn div(self, rhs: f64) -> Self {
    let mut v = self;
    v /= rhs;
    v
  }
}

/// Overriding ^= for vector cross product.
impl BitXorAssign for Vector3 {
  fn bitxor_assign(&mut self, rhs: Self) {
    self.v = [
        self.y() * rhs.z() - self.z() * rhs.y(),
        self.z() * rhs.x() - self.x() * rhs.z(),
        self.x() * rhs.y() - self.y() * rhs.x()
    ];
  }
}

/// Overriding ^ for vector cross product.
impl BitXor for Vector3 {
  type Output = Self;
  fn bitxor(self, rhs: Self) -> Self {
    let mut v = self;
    v ^= rhs;
    v
  }
}

/// Overriding [] for immutable indexing.
impl Index<usize> for Vector3 {
  type Output = f64;
  fn index(&self, ind: usize) -> &f64 {
    assert!(ind < 3, "Index out of bound!");
    &self.v[ind]
  }
}

/// Overriding [] for mutable indexing.
impl IndexMut<usize> for Vector3 {
  fn index_mut(&mut self, ind: usize) -> &mut f64 {
    assert!(ind < 3, "Index out of bound!");
    &mut self.v[ind]
  }
}

/// Overriding == and != to allow comparison within error bounds.
impl PartialEq for Vector3 {
  fn eq(&self, rhs: &Self) -> bool {
    (self.v[0] - rhs.v[0]).abs() < constants::EPSILON
        && (self.v[1] - rhs.v[1]).abs() < constants::EPSILON
        && (self.v[2] - rhs.v[2]).abs() < constants::EPSILON
  }

  fn ne(&self, rhs: &Self) -> bool {
    !self.eq(rhs)
  }
}

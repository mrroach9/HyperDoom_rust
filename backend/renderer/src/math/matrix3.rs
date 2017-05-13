use common::constants;
use std::fmt;
use std::mem;
use std::ops::{
  Add, AddAssign, Sub, SubAssign, Neg,
  Div, DivAssign, Mul, MulAssign,
  Index, IndexMut,
};
use std::cmp::PartialEq;
use math::Vector3;

/// Definition and operations of a 3x3 double-precision matrix.
#[derive(Debug, Copy, Clone)]
pub struct Matrix3 {
  m: [Vector3; 3],
}

impl Matrix3 {
  /// Returns a new matrix whose value is the transpose of this.
  pub fn t(&self) -> Matrix3 {
    let mut m = *self;
    m.transpose_self();
    m
  }

  /// Converts this matrix to its transpose.
  pub fn transpose_self(&mut self) {
    let mut t = self.m[0][1];
    self.m[0][1] = self.m[1][0];
    self.m[1][0] = t;
    t = self.m[0][2];
    self.m[0][2] = self.m[2][0];
    self.m[2][0] = t;
    t = self.m[1][2];
    self.m[1][2] = self.m[2][1];
    self.m[2][1] = t;
  }

  /// Calculates the determinant.
  pub fn det(&self) -> f64 {
      self.m[0][0] * self.m[1][1] * self.m[2][2]
          + self.m[0][1] * self.m[1][2] * self.m[2][0]
          + self.m[0][2] * self.m[1][0] * self.m[2][1]
          - self.m[2][0] * self.m[1][1] * self.m[0][2]
          - self.m[1][2] * self.m[2][1] * self.m[0][0]
          - self.m[0][1] * self.m[1][0] * self.m[2][2]
  }

  /// Initializes from three vectors, each vector will become a row in the
  /// matrix, by the order they are specified.
  pub fn new_from_vectors(v1: Vector3, v2: Vector3, v3: Vector3) -> Self {
    Self {
      m: [v1, v2, v3],
    }
  }

  /// Initializes from nine floating numbers.
  pub fn new(m11: f64, m12: f64, m13: f64,
      m21: f64, m22: f64, m23: f64,
      m31: f64, m32: f64, m33: f64) -> Self {
    Self::new_from_vectors(
        Vector3::new(m11, m12, m13),
        Vector3::new(m21, m22, m23),
        Vector3::new(m31, m32, m33))
  }

  /// Initializes from three arrays. Same as new_from_vectors.
  pub fn new_from_arrays(a1: &[f64; 3], a2: &[f64; 3], a3: &[f64; 3]) -> Self {
    Self::new_from_vectors(
        Vector3::new_from_array(a1),
        Vector3::new_from_array(a2),
        Vector3::new_from_array(a3))
  }

  /// Initializes a diagnal matrix from a vector.
  pub fn diag(v: Vector3) -> Self {
    Self::new(
        v[0], 0.0, 0.0,
        0.0, v[1], 0.0,
        0.0, 0.0, v[2])
  }

  /// Initializes an identity matrix.
  pub fn identity() -> Self {
    Self::diag(Vector3::one())
  }

  /// Initializes a zero matrix.
  pub fn zero() -> Self {
    Self::diag(Vector3::zero())
  }

  /// Creates the cross-product-equivalent matrix from a given vector.
  /// E.g. a^b = M_a * b, calling this method with param a returns M_a.
  pub fn cross_prod_mat(v: Vector3) -> Self {
    Self::new(
        0.0, -v.z(), v.y(),
        v.z(), 0.0, -v.x(), 
        -v.y(), v.x(), 0.0)
  }
}

/// Overriding += for matrix addition.
impl AddAssign for Matrix3 {
  fn add_assign(&mut self, rhs: Self) {
    self.m[0] += rhs.m[0];
    self.m[1] += rhs.m[1];
    self.m[2] += rhs.m[2];
  }
}

/// Overriding + for matrix addition.
impl Add for Matrix3 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    let mut m = self;
    m += rhs;
    m
  }
}

/// Overriding -= for matrix subtraction.
impl SubAssign for Matrix3 {
  fn sub_assign(&mut self, rhs: Self) {
    self.m[0] -= rhs.m[0];
    self.m[1] -= rhs.m[1];
    self.m[2] -= rhs.m[2];
  }
}

/// Overriding - for matrix subtraction.
impl Sub for Matrix3 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    let mut m = self;
    m -= rhs;
    m
  }
}

/// Overriding - for matrix negation.
impl Neg for Matrix3 {
  type Output = Self;
  fn neg(self) -> Self {
    Matrix3::zero() - self
  }
}

/// Overriding *= for matrix multiplication with a numerical.
impl MulAssign<f64> for Matrix3 {
  fn mul_assign(&mut self, rhs: f64) {
    self.m[0] *= rhs;
    self.m[1] *= rhs;
    self.m[2] *= rhs;
  }
}

/// Overriding *= for matrix multiplication with a matrix of the same dimension.
impl MulAssign for Matrix3 {
  fn mul_assign(&mut self, rhs: Self) {
    self.m = (*self * rhs).m
  }
}

/// Overriding * for matrix multiplication with a numerical.
impl Mul<f64> for Matrix3 {
  type Output = Self;
  fn mul(self, rhs: f64) -> Self {
    let mut m = self;
    m *= rhs;
    m
  }
}

/// Overriding * for matrix multiplication with a vector.
impl Mul<Vector3> for Matrix3 {
  type Output = Vector3;
  fn mul(self, rhs: Vector3) -> Vector3 {
    Vector3::new(self.m[0] * rhs, self.m[1] * rhs, self.m[2] * rhs)
  }
}

/// Overriding * for matrix multiplication with a matrix of the same dimension.
impl Mul for Matrix3 {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self {
    let rt = rhs.t();
    Matrix3::new_from_vectors(
      self * rt.m[0], self * rt.m[1], self * rt.m[2]).t()
  }
}

/// Overriding /= for matrix division with a numerical.
impl DivAssign<f64> for Matrix3 {
  fn div_assign(&mut self, rhs: f64) {
    assert!(rhs.abs() > constants::EPSILON_TINY, "Division by zero!");
    *self *= 1.0 / rhs;
  }
}

/// Overriding / for matrix division with a numerical.
impl Div<f64> for Matrix3 {
  type Output = Self;
  fn div(self, rhs: f64) -> Self {
    let mut m = self;
    m /= rhs;
    m
  } 
}

/// Overriding [] for immutable indexing.
impl Index<usize> for Matrix3 {
  type Output = Vector3;
  fn index(&self, ind: usize) -> &Vector3 {
    assert!(ind < 3, "Index out of bound!");
    &self.m[ind]
  }
}

/// Overriding [] for mutable indexing.
impl IndexMut<usize> for Matrix3 {
  fn index_mut(&mut self, ind: usize) -> &mut Vector3 {
    assert!(ind < 3, "Index out of bound!");
    &mut self.m[ind]
  }
}

/// Overriding == and != to allow comparison within error bounds.
impl PartialEq for Matrix3 {
  fn eq(&self, rhs: &Self) -> bool {
    self.m[0] == rhs.m[0] && self.m[1] == rhs.m[1] && self.m[2] == rhs.m[2]
  }

  fn ne(&self, rhs: &Self) -> bool {
    !self.eq(rhs)
  }
}

/// Printing the matrix as
/// (
///  (m11, m12, m13),
///  (m21, m22, m23),
///  (m31, m32, m33)
/// )
impl fmt::Display for Matrix3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(\n {},\n {},\n {}\n)", self.m[0], self.m[1], self.m[2])
  }
}

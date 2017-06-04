use common::constants;

// Calculate max of three float values. Due to undefined behaviors of Inf and
// NaN, max and min of floating numbers in an array are not implemented, hereby
// we implement them.
pub fn max(v1: f64, v2: f64, v3: f64) -> f64 {
  max_array(&[v1, v2, v3])
}

// Calculate min of three float values. Due to undefined behaviors of Inf and
// NaN, max and min of floating numbers in an array are not implemented, hereby
// we implement them.
pub fn min(v1: f64, v2: f64, v3: f64) -> f64 {
  min_array(&[v1, v2, v3])
}

// Calculate max of an array of floats. Due to undefined behaviors of Inf and
// NaN, max and min of floating numbers in an array are not implemented, hereby
// we implement them.
pub fn max_array(v: &[f64]) -> f64 {
  v.iter().cloned().fold(-constants::INFINITY, f64::max)
}

// Calculate min of an array of floats. Due to undefined behaviors of Inf and
// NaN, max and min of floating numbers in an array are not implemented, hereby
// we implement them.
pub fn min_array(v: &[f64]) -> f64 {
  v.iter().cloned().fold(constants::INFINITY, f64::min)
}

// Compare two floats within a small error bound.
pub fn equal_with_bound(lhs: f64, rhs: f64) -> bool {
  lhs - rhs <= constants::EPSILON && rhs - lhs <= constants::EPSILON
}

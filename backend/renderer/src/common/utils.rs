use common::constants;

pub fn max(v1: f64, v2: f64, v3: f64) -> f64 {
  max_array(&[v1, v2, v3])
}

pub fn min(v1: f64, v2: f64, v3: f64) -> f64 {
  min_array(&[v1, v2, v3])
}

pub fn max_array(v: &[f64]) -> f64 {
  v.iter().cloned().fold(-constants::INFINITY, f64::max)
}

pub fn min_array(v: &[f64]) -> f64 {
  v.iter().cloned().fold(-constants::INFINITY, f64::min)
}

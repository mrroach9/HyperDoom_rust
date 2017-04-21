use std::fmt;

#[derive(Debug)]
pub struct Vector3 {
  v: [f64; 3],
}

impl Vector3 {
  pub fn x(&self) -> f64 {
    self.v[0]
  }

  pub fn y(&self) -> f64 {
    self.v[1]
  }

  pub fn z(&self) -> f64 {
    self.v[2]
  }

  pub fn new() -> Vector3 {
    Vector3 {
      v: [0.0, 0.0, 0.0],
    }
  }
}

impl fmt::Display for Vector3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {})", self.v[0], self.v[1], self.v[2])
  }
}

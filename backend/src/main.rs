extern crate renderer;

use renderer::math::{Vector3, Matrix3};
use renderer::geometry::{BoundingBox3, HasVolume};
use renderer::common::constants;

fn main() {
  println!("Hello, world!");
  println!("=============Testing constants...==============");
  println!("EPSILON = {}, EPSILON_TINY = {}",
      constants::EPSILON, constants::EPSILON_TINY);

  println!("=============Testing Vector3...==============");  
  let v = Vector3::new(1.0, 2.0, 3.0);
  println!("v = {}", &v);
  println!("|v| = {}", &v.len());

  println!("=============Testing Matrix3...==============");
  let m = Matrix3::new(
      1.0, 2.0, 3.0,
      4.0, 5.0, 6.0,
      7.0, 8.0, -1.0
  );
  println!("m = {}", &m);
  println!("|m| = {}", &m.det());

  println!("============Testing BoundingBox3...==========");
  let b = BoundingBox3::new(
      Vector3::new(1.0, 2.0, 3.0),
      Vector3::new(11.0, 22.0, 5.0));
  println!("b = {}", &b);
  println!("volume of b = {}", &b.volume());
}

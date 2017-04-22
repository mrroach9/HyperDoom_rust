extern crate renderer;

use renderer::math::Vector3;
use renderer::common::constants;

fn main() {
    println!("Hello, world!");
    let v = Vector3::new(1.0, 2.0, 3.0);
    println!("v = {}", &v);
}

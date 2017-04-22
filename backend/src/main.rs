extern crate renderer;

use renderer::math::Vector3;
use renderer::common::constants;

fn main() {
    println!("Hello, world!");
    let mut v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::one();
    println!("v1 = {}", &v1);
    println!("v2 = {}", &v2);
    let v3 = v1 + v2;
    println!("v3 = v1 + v2, v3 = {}", &v3);
    println!("v1 + v2 = {}", v1 + v2);
    println!("v1 = {}", &v1);
    println!("v2 = {}", &v2);
    println!("v1 - v2 = {}", v1 - v2);
    println!("v1 * 2 = {}", v1 * 2.0);
    println!("v1 / 3 = {}", v1 / 3.0);
    println!("v1 * v2 = {}", v1 * v2);
    println!("v1 ^ v2 = {}", v1 ^ v2);
    println!("v1[1] = {}", v1[1]);
    v1[1] = 10.1123;
    println!("v1[1] = 10.1123, v1 = {}", &v1);
    println!("INFINITY = {}", constants::INFINITY);
}

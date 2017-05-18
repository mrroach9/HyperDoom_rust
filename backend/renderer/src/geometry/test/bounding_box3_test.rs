use super::super::{BoundingBox3, HasVolume, HasSurfaceArea};
use super::super::super::math::Vector3;

#[test]
fn test_init_works() {
  let b1 = BoundingBox3::new(
      Vector3::new(0.0, 1.0, 2.0),
      Vector3::new(10.0, 21.0, 32.0));
  let b2 = BoundingBox3::new_from_nums(0.0, 1.0, 2.0, 10.0, 21.0, 32.0);
  assert!(b1 == b2);
  assert!(b2.min_corner() == Vector3::new(0.0, 1.0, 2.0));
  assert!(b2.max_corner() == Vector3::new(10.0, 21.0, 32.0));
}

#[test]
fn test_basic_properties() {
  let b = BoundingBox3::new(
      Vector3::new(0.0, 1.0, 2.0),
      Vector3::new(10.0, 21.0, 32.0));
  assert_eq!(b.size(), Vector3::new(10.0, 20.0, 30.0));
  assert_eq!(b.volume(), 10.0 * 20.0 * 30.0);
  assert_eq!(b.surface_area(),
      2200.0 /* 2 * 10 * 20 + 2 * 10 * 30 + 2 * 20 * 30 */);

  let box_shift = b.move_by(Vector3::new(10.0, 10.0, 20.0));
  assert_eq!(b.size(), box_shift.size());
  assert_eq!(b.volume(), box_shift.volume());
  assert_eq!(b.surface_area(), box_shift.surface_area());

  let box_expected = BoundingBox3::new(
      Vector3::new(10.0, 11.0, 22.0),
      Vector3::new(20.0, 31.0, 52.0));
  assert_eq!(box_shift, box_expected);
}

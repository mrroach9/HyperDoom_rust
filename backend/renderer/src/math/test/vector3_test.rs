use super::super::Vector3;

#[test]
fn inits_correctly() {
  let v1 = Vector3::zero();
  assert_eq!(v1.x(), 0.0);
  assert_eq!(v1.y(), 0.0);
  assert_eq!(v1.z(), 0.0);

  let v2 = Vector3::new(1.0, 2.0, 3.0);
  assert_eq!(v2.x(), 1.0);
  assert_eq!(v2.y(), 2.0);
  assert_eq!(v2.z(), 3.0);

  let v3 = Vector3::identity(2.0);
  assert_eq!(v3.x(), 2.0);
  assert_eq!(v3.y(), 2.0);
  assert_eq!(v3.z(), 2.0);
}

#[test]
fn basic_operators_work() {
  let mut u = Vector3::identity(0.3);
  let mut v = Vector3::new(1.0, 1.0, 1.0);
  let w = v.clone();

  assert!(v == w);
  assert!(u != v);
  assert!(v == Vector3::identity(1.0));
  assert!(v == Vector3::one());

  v *= 0.3;
  assert!(u == v);
  assert!(w * 0.3 == u);
  u /= 0.3;
  assert!(u == w);
  assert!(v / 0.3 == u);

  v = u.clone();
  v += Vector3::new(1e-9, 2e-9, 1e-8);
  assert!(u == v);

  v = Vector3::new(1.0, 2.0, 3.0);
  u = Vector3::new(2.0, 5.0, 8.0);
  assert!(u + v == Vector3::new(3.0, 7.0, 11.0));
  assert!(u - v == Vector3::new(1.0, 3.0, 5.0));
  u += v;
  assert_eq!(u.x(), 3.0);
  assert_eq!(u.y(), 7.0);
  assert_eq!(u.z(), 11.0);
}

#[test]
fn more_operators_work() {
  let mut u = Vector3::new(3.0, -4.0, 0.0);
  let v = Vector3::new(1.0, 2.0, 3.0);
  assert_eq!(u * v, -5.0); // 1 * 3 + 2 * (-4) + 3 * 0
  assert_eq!(u.len(), 5.0);
  assert_eq!(u.len2(), 25.0);
  assert!((u ^ v) == Vector3::new(-12.0, -9.0, 10.0));
  assert!((v ^ u) == Vector3::new(12.0, 9.0, -10.0));
  assert!((u ^ u) == Vector3::zero());

  assert!(u.normalize() == Vector3::new(0.6, -0.8, 0.0));
  assert!(Vector3::zero().normalize() == Vector3::new(0.0, 0.0, 0.0));
  let l = u.normalize_self();
  assert_eq!(l, 5.0);
  assert!(u == Vector3::new(0.6, -0.8, 0.0));
}

#[test]
fn indexing_works() {
  let mut v = Vector3::new(1.0, 2.0, 3.0);
  assert_eq!(v[0], 1.0);
  assert_eq!(v[1], 2.0);
  assert_eq!(v[2], 3.0);
  let a = v.to_array();
  assert_eq!(a[0], 1.0);
  assert_eq!(a[1], 2.0);
  assert_eq!(a[2], 3.0);
  v[0] = 11.0;
  v[1] = 12.0;
  v[2] = 13.0;
  assert!(v == Vector3::new(11.0, 12.0, 13.0));
}

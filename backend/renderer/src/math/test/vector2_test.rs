use super::super::Vector2;

#[test]
fn inits_correctly() {
  let v1 = Vector2::zero();
  assert_eq!(v1.u(), 0.0);
  assert_eq!(v1.v(), 0.0);

  let v2 = Vector2::new(1.0, 2.0);
  assert_eq!(v2.u(), 1.0);
  assert_eq!(v2.v(), 2.0);
}

#[test]
fn basic_operators_work() {
  let mut u = Vector2::new(0.3, 0.3);
  let mut v = Vector2::new(1.0, 1.0);
  let w = v.clone();

  assert!(v == w);
  assert!(u != v);

  v *= 0.3;
  assert!(u == v);
  assert!(w * 0.3 == u);
  u /= 0.3;
  assert!(u == w);
  assert!(v / 0.3 == u);

  v = u.clone();
  v += Vector2::new(1e-9, 2e-9);
  assert!(u == v);

  v = Vector2::new(1.0, 2.0);
  u = Vector2::new(2.0, 5.0);
  assert!(u + v == Vector2::new(3.0, 7.0));
  assert!(u - v == Vector2::new(1.0, 3.0));
  u += v;
  assert_eq!(u.u(), 3.0);
  assert_eq!(u.v(), 7.0);
}

#[test]
fn indexing_works() {
  let mut v = Vector2::new(1.0, 2.0);
  assert_eq!(v[0], 1.0);
  assert_eq!(v[1], 2.0);
  let a = v.to_array();
  assert_eq!(a[0], 1.0);
  assert_eq!(a[1], 2.0);
  v[0] = 11.0;
  v[1] = 12.0;
  assert!(v == Vector2::new(11.0, 12.0));
}

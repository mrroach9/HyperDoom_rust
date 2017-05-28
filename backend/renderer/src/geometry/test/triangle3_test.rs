use super::super::{
    Triangle3, HasBoundingBox3, BoundingBox3, HasSurfaceArea
};
use math::Vector3;

struct TestBed {
  tri1: Triangle3,
  tri2: Triangle3,
}

fn create_test_bed() -> TestBed {
  let tri1 = Triangle3::new(
    Vector3::new(0.0, 0.0, 0.0),
    Vector3::new(1.0, 0.0, 0.0),
    Vector3::new(0.0, 2.0, 0.0)
  );
  let tri2 = Triangle3::new(
    Vector3::x_unit(),
    Vector3::y_unit(),
    Vector3::z_unit()
  );
  TestBed {
    tri1: tri1,
    tri2: tri2
  }
}

fn dispose_test_bed(tb: &mut TestBed) {}

#[test]
fn test_init_and_getters() {
  let mut tb = create_test_bed();

  let almost_zero_tri = Triangle3::new(
    Vector3::new(0.0, -1e-9, 0.0),
    Vector3::new(1.0, 0.0, 0.0),
    Vector3::new(0.0, 2.0, 1e-9)
  );

  assert!(almost_zero_tri == tb.tri1);
  assert!(almost_zero_tri != tb.tri2);

  assert_eq!(tb.tri1.v(2), Vector3::new(0.0, 2.0, 0.0));
  assert_eq!(tb.tri1.e(1), Vector3::new(-1.0, 2.0, 0.0));
  assert_eq!(tb.tri2.e(2), Vector3::new(1.0, 0.0, -1.0));

  dispose_test_bed(&mut tb);
}

#[test]
fn test_properties() {
  let mut tb = create_test_bed();
  
  assert_eq!(tb.tri1.normal(), Vector3::new(0.0, 0.0, 1.0));
  assert_eq!(tb.tri2.normal(), Vector3::identity(3.0f64.sqrt() / 3.0));

  assert_eq!(tb.tri1.bounding_box3(),
      BoundingBox3::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 2.0, 0.0)));
  assert_eq!(tb.tri2.bounding_box3(),
      BoundingBox3::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 1.0, 1.0)));

  // 1 * 2 / 2 = 1.0
  assert_eq!(tb.tri1.surface_area(), 1.0);
  // sqrt(3) / 4 * (sqrt(2) ^ 2)
  assert_eq!(tb.tri2.surface_area(), 3.0f64.sqrt() / 2.0);

  dispose_test_bed(&mut tb);
}

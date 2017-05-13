use super::super::{Matrix3, Vector3};

struct TestBed {
  m1: Matrix3,
  m2: Matrix3,
}

fn create_test_bed() -> TestBed {
  let m1;
  let m2;
  m1 = Matrix3::new(
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0,
    7.0, 8.0, 9.0
  );
  m2 = Matrix3::new(
    0.0, -1.0, 0.0,
    2.0, 1000.0, 0.0003,
    3.0, 0.0, -1000000.0
  );
  TestBed {
    m1: m1,
    m2: m2,
  }
}

fn dispose_test_bed(tb: &mut TestBed) {}

#[test]
fn init_works() {
  let mut tb = create_test_bed();

  let mat1 = Matrix3::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
  assert_eq!(mat1, Matrix3::zero());

  let mat2 = Matrix3::new_from_arrays(
      &[1.0, 0.0, 0.0],
      &[0.0, 1.0, 0.0],
      &[0.0, 0.0, 1.0]
  );
  let mat3 = Matrix3::new_from_vectors(
      Vector3::x_unit(),
      Vector3::y_unit(),
      Vector3::z_unit()
  );
  assert_eq!(mat2, mat3);
  assert_eq!(mat2, Matrix3::identity());
  assert_eq!(mat3, Matrix3::diag(Vector3::new(1.0, 1.0, 1.0)));

  dispose_test_bed(&mut tb);
}

#[test]
fn test_basic_operators() {
  let mut tb = create_test_bed();
  
  // m3 == m1 + m2.
  let m3 = Matrix3::new_from_arrays(
      &[1.0, 1.0, 3.0],
      &[6.0, 1005.0, 6.0003],
      &[10.0, 8.0, -999991.0]
  );
  assert_eq!(tb.m1 + tb.m2, m3);
  assert_eq!(m3 - tb.m2, tb.m1);
  assert_eq!(m3 * 2.0 - tb.m2 * 2.0, tb.m1 * 2.0);
  tb.m1 += tb.m2;
  assert_eq!(tb.m1, m3);
  tb.m1 -= tb.m2;
  assert_eq!(tb.m1 + tb.m2, m3);
  assert_eq!(m3 + Matrix3::diag(Vector3::identity(1e-9)), m3);
  assert_eq!(m3 / 2.0, m3 * 0.5);

  dispose_test_bed(&mut tb);  
}

#[test]
fn getters_and_setters_via_indices() {
  let mut tb = create_test_bed();

  assert_eq!(tb.m1[0][0], 1.0);
  assert_eq!(tb.m1[2][1], 8.0);
  let v = Vector3::new(-1.0, -2.0, -3.0);
  tb.m1[2] = v;
  assert_eq!(tb.m1[2][0], -1.0);
  assert_eq!(tb.m1[2][1], -2.0);
  tb.m1[1][0] = -100.0;
  assert_eq!(tb.m1[1], Vector3::new(-100.0, 5.0, 6.0));

  dispose_test_bed(&mut tb);
}

#[test]
fn test_advanced_operations() {
  let mut tb = create_test_bed();

  assert_eq!(tb.m1 * Matrix3::identity(), tb.m1);
  assert_eq!(Matrix3::identity() * tb.m1, tb.m1);
  assert_eq!(tb.m1 * Matrix3::diag(Vector3::identity(3.0)), tb.m1 * 3.0);
  assert_eq!(tb.m1 * tb.m2, (tb.m2.t() * tb.m1.t()).t());
  let m3 = Matrix3::new_from_arrays(
      &[2.0, 3.0, -1.0],
      &[1.0, 0.0, 4.0],
      &[-2.0, 2.0, 10.0],
  );
  // m4 == m1 * m3.
  let m4 = Matrix3::new_from_arrays(
      &[-2.0, 9.0, 37.0],
      &[1.0, 24.0, 76.0],
      &[4.0, 39.0, 115.0],
  );
  assert_eq!(tb.m1 * m3, m4);
  // m5 == m1^T
  let mut m5 = Matrix3::new_from_arrays(
      &[1.0, 4.0, 7.0],
      &[2.0, 5.0, 8.0],
      &[3.0, 6.0, 9.0],
  );
  assert_eq!(tb.m1.t(), m5);
  // Verifies that transpose is self-reflective.
  assert_eq!(m5.t(), tb.m1);
  assert_eq!(tb.m1.t().t(), tb.m1);
  m5.transpose_self();
  assert_eq!(tb.m1, m5);
  // Verifies that (A*B)^T = B^T * A^T.
  assert_eq!(m3.t() * tb.m1.t(), m4.t());

  // Verifies the value of determinant calculation.
  assert_eq!(tb.m1.det(), 0.0);
  tb.m1[2][2] = 10.0;
  assert_eq!(tb.m1.det(), -3.0);
  assert_eq!(Matrix3::diag(Vector3::identity(3.0)).det(), 27.0);

  dispose_test_bed(&mut tb);
}

#[test]
fn test_cross_prod_mat() {
  let mut tb = create_test_bed();

  let v1 = Vector3::new(1.0, -1.0, 2.0);
  let v2 = Vector3::new(-0.5, 0.0, 1.0);
  // v3 = v1 ^ v2.
  let v3 = Vector3::new(-1.0, -2.0, -0.5);
  assert_eq!(Matrix3::cross_prod_mat(v1) * v2, v3);
  assert_eq!(Matrix3::cross_prod_mat(v1) * v2, v1 ^ v2);

  dispose_test_bed(&mut tb);
}

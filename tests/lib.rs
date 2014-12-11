extern crate num_rs;
use num_rs::matrix;

#[test]
fn test_basic_matrix() {
  let mut m = matrix::Matrix::new(2, 2, 0.0);
  // test is rows and columns are as expected
  assert_eq!(m.num_rows(), 2);
  assert_eq!(m.num_cols(), 2);

  // test for correctness of values
  for i in range(0, m.num_rows()) {
    for j in range(0, m.num_cols()) {
      m.set(i, j, (i + j) as f64);
    }
  }

  for i in range(0, m.num_rows()) {
    for j in range(0, m.num_cols()) {
      assert_eq!(m.get(i, j), (i + j) as f64);
    }
  }
}

#[test]
fn test_matrix_add() {
  let mut m1 = matrix::Matrix::new(2, 2, 0.0);
  let mut m2 = matrix::Matrix::new(2, 2, 0.0);
  for i in range(0, m1.num_rows()) {
    for j in range(0, m1.num_cols()) {
      m1.set(i, j, (i + j) as f64);
      m2.set(i, j, (i + j) as f64);
    }
  }

  let res = m1.add(m2);
  for i in range(0, res.num_rows()) {
    for j in range(0, res.num_cols()) {
      assert_eq!(res.get(i, j), 2.0 * ((i + j) as f64));
    }
  }
}
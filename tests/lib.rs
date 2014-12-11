#![feature(macro_rules)]

extern crate num_rs;
use num_rs::matrix::Matrix;
use num_rs::solver;
use std::num::abs_sub;

macro_rules! assert_nearby(
  ($lhs:expr, $rhs:expr) => (
    assert!(abs_sub($lhs, $rhs) < 0.0001);
  );
)

#[test]
fn test_basic_matrix() {
  let mut m = Matrix::new(2, 2, 0.0);
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
fn test_identity_matrix() {
  let m = Matrix::identity(5);
  for i in range(0, 5) {
    for j in range(0, 5) {
      if i == j {
        assert_eq!(m.get(i, j), 1.0);
      } else {
        assert_eq!(m.get(i, j), 0.0);
      }
    }
  }
}

#[test]
fn test_matrix_add() {
  let mut m1 = Matrix::new(2, 2, 0.0);
  let mut m2 = Matrix::new(2, 2, 0.0);
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

#[test]
fn test_matrix_multiply() {
  let mut m1 = Matrix::new(2, 2, 0.0);
  let mut m2 = Matrix::new(2, 2, 0.0);
  for i in range(0, m1.num_rows()) {
    for j in range(0, m1.num_cols()) {
      m1.set(i, j, (i + j) as f64);
      m2.set(i, j, (i + j) as f64);
    }
  }

  let res = m1.multiply(m2);
  assert_eq!(res.get(0, 0), 1.0);
  assert_eq!(res.get(0, 1), 2.0);
  assert_eq!(res.get(1, 0), 2.0);
  assert_eq!(res.get(1, 1), 5.0);
}

#[test]
fn test_scalar_multiply() {
  let mut m = Matrix::new(2, 2, 0.0);
  for i in range(0, m.num_rows()) {
    for j in range(0, m.num_cols()) {
      m.set(i, j, (i + j) as f64);
    }
  }

  m.scalar_multiply(-1.0);
  for i in range(0, m.num_rows()) {
    for j in range(0, m.num_cols()) {
      assert_eq!(m.get(i, j), -1.0 * ((i + j) as f64));
    }
  }
}

#[test]
fn test_gauss_seidel() {
  let mut a = Matrix::new(2, 2, 0.0);
  a.set(0, 0, 16.0);
  a.set(0, 1, 3.0);
  a.set(1, 0, 7.0);
  a.set(1, 1, -11.0);
  let mut b = Matrix::new(2, 1, 0.0);
  b.set(0, 0, 11.0);
  b.set(1, 0, 13.0);
  let mut x = Matrix::new(2, 1, 1.0);
  x = solver::gauss_seidel(a, b, x, 1000);
  assert_nearby!(x.get(0, 0), 0.8122);
  assert_nearby!(x.get(1, 0), -0.6650);
}

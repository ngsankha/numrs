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
fn test_gauss_seidel() {
  let a_elems = [16.0, 3.0, 7.0, -11.0];
  let b_elems = [11.0, 13.0];
  let a = Matrix::from_elems(2, 2, a_elems.as_slice());
  let b = Matrix::from_elems(2, 1, b_elems.as_slice());
  let mut x = Matrix::new(2, 1, 1.0);
  x = solver::gauss_seidel(a, b, x, 1000);
  assert_nearby!(x.get(0, 0), 0.8122);
  assert_nearby!(x.get(1, 0), -0.6650);
}

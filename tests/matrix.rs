extern crate numrs;
use numrs::matrix;
use numrs::matrix::Matrix;

#[test]
fn test_basic_matrix() {
  let mut m = Matrix::new(2, 2, 0.0);
  assert_eq!(m.num_rows(), 2);
  assert_eq!(m.num_cols(), 2);
  m.set(1, 1, 1.0);
  assert_eq!(m.get(0, 0), 0.0);
  assert_eq!(m.get(0, 1), 0.0);
  assert_eq!(m.get(1, 0), 0.0);
  assert_eq!(m.get(1, 1), 1.0);
}

#[test]
fn test_identity_matrix() {
  let m = matrix::identity(5);
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
  let elems = [1.0, 2.0, 3.0, 4.0];
  let m1 = matrix::from_elems(2, 2, elems.as_slice());
  let m2 = matrix::from_elems(2, 2, elems.as_slice());
  let res = m1 + m2;

  assert_eq!(res[0][0], 2.0);
  assert_eq!(res[0][1], 4.0);
  assert_eq!(res[1][0], 6.0);
  assert_eq!(res[1][1], 8.0);
}

#[test]
fn test_matrix_multiply() {
  let elems = [1.0, 2.0, 3.0, 4.0];
  let m1 = matrix::from_elems(2, 2, elems.as_slice());
  let m2 = matrix::from_elems(2, 2, elems.as_slice());
  let res = m1 * m2;

  assert_eq!(res[0][0], 7.0);
  assert_eq!(res[0][1], 10.0);
  assert_eq!(res[1][0], 15.0);
  assert_eq!(res[1][1], 22.0);
}

#[test]
fn test_scalar_multiply() {
  let elems = [1.0, 2.0, 3.0, 4.0];
  let m = matrix::from_elems(2, 2, elems.as_slice());

  let mut m_ = -m.clone();
  assert_eq!(m_[0][0], -1.0);
  assert_eq!(m_[0][1], -2.0);
  assert_eq!(m_[1][0], -3.0);
  assert_eq!(m_[1][1], -4.0);

  m_ = m.clone() * 2.0;
  assert_eq!(m_[0][0], 2.0);
  assert_eq!(m_[0][1], 4.0);
  assert_eq!(m_[1][0], 6.0);
  assert_eq!(m_[1][1], 8.0);
}

#[test]
fn test_matrix_index() {
  let elems = [16.0, 3.0, 7.0, -11.0];
  let mat = matrix::from_elems(2, 2, elems.as_slice());
  assert_eq!(mat[0].len(), 2);
  assert_eq!(mat[0][0], 16.0);
  assert_eq!(mat[0][1], 3.0);
}

#[test]
fn test_matrix_clone() {
  let elems = [16.0, 3.0, 7.0, -11.0];
  let mat = matrix::from_elems(2, 2, elems.as_slice());
  let m = mat.clone();
  assert_eq!(m[0][0], 16.0);
  assert_eq!(m[0][1], 3.0);
  assert_eq!(m[1][0], 7.0);
  assert_eq!(m[1][1], -11.0);
}

#[test]
fn test_matrix_transpose() {
  let elems = [1.0, 2.0, 3.0, 4.0];
  let mut mat = matrix::from_elems(2, 2, elems.as_slice());
  mat.transpose();
  assert_eq!(mat[0][0], 1.0);
  assert_eq!(mat[0][1], 3.0);
  assert_eq!(mat[1][0], 2.0);
  assert_eq!(mat[1][1], 4.0);
}

#[test]
fn test_matrix_reshape() {
  let elems = [1.0, 2.0, 3.0, 4.0];
  let mut mat = matrix::from_elems(2, 2, elems.as_slice());
  mat.reshape(1, 4);
  assert_eq!(mat[0][0], 1.0);
  assert_eq!(mat[0][1], 2.0);
  assert_eq!(mat[0][2], 3.0);
  assert_eq!(mat[0][3], 4.0);
}

#[test]
fn test_matrix_eq() {
  let elem1 = [1.0, 2.0, 3.0, 4.0];
  let m1 = matrix::from_elems(2, 2, elem1.as_slice());
  let elem2 = [1.0, 3.0, 3.0, 4.0];
  let m2 = matrix::from_elems(2, 2, elem2.as_slice());
  assert!(m1 != m2)
}

#[test]
fn test_matrix_trace() {
  let elems = [1.0, 2.0, 3.0, 4.0];
  let mat = matrix::from_elems(2, 2, elems.as_slice());
  assert_eq!(mat.trace(), 5.0);
}

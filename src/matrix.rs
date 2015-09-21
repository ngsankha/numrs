use std::ops::{Index, Add, Sub, Mul, Neg};

/// A 2D Matrix type composed of `f64` elements.
///
/// The Matrix type supports simple arithmetic operations from the start and
/// provides convenient methods for accessing the elements of the matrix.
///
/// # Examples
/// ```
/// use numrs::matrix;
/// use numrs::matrix::Matrix;
///
/// // Create a new 2x2 matrix with all values 0.0
/// let m1 = Matrix::new(2, 2, 0.0);
///
/// // Creates a 2x2 matrix from the above vector in row major order
/// let elems = [1.0, 2.0, 3.0, 4.0];
/// let m2 = matrix::from_elems(2, 2, &elems);
///
/// let mut res = m1.clone() + m2.clone(); // add two matrices
/// res = m1.clone() - m2.clone(); // subtract 2 matrices
/// res = m1.clone() * m2.clone(); // matrix product of 2 matrices
/// res = m2.clone() * 5.0; // scalar product of a matrix
/// ```

pub struct Matrix {
  rows: usize,
  cols: usize,
  data: Vec<f64>
}

impl Index<usize> for Matrix {
  type Output = [f64];

  #[inline]
  fn index<'a>(&'a self, index: usize) -> &'a [f64] {
    &self.data[self.rows * index..self.rows * index + self.cols]
  }
}

impl Clone for Matrix {
  fn clone(&self) -> Matrix {
    Matrix {
      rows: self.num_rows(),
      cols: self.num_cols(),
      data: self.data.clone()
    }
  }

  fn clone_from(&mut self, source: &Matrix) {
    self.rows = source.num_rows();
    self.cols = source.num_cols();
    self.data = source.data.clone();
  }
}

impl Add<Matrix> for Matrix {
  type Output = Matrix;

  fn add(self, rhs: Matrix) -> Matrix {
    if self.num_rows() == rhs.num_rows() && self.num_cols() == rhs.num_cols() {
      let mut new_mat = Matrix::new(self.num_rows(), self.num_cols(), 0.0);
      for i in 0..self.data.len() {
        new_mat.data[i] = self.data[i] + rhs.data[i];
      }
      new_mat
    } else {
      panic!("Matrices are not conformable for addition.");
    }
  }
}

impl Sub<Matrix> for Matrix {
  type Output = Matrix;

  fn sub(self, rhs: Matrix) -> Matrix {
    if self.num_rows() == rhs.num_rows() && self.num_cols() == rhs.num_cols() {
      let mut new_mat = Matrix::new(self.num_rows(), self.num_cols(), 0.0);
      for i in 0..self.data.len() {
        new_mat.data[i] = self.data[i] - rhs.data[i];
      }
      new_mat
    } else {
      panic!("Matrices are not conformable for subtraction.");
    }
  }
}

impl Mul<Matrix> for Matrix {
  type Output = Matrix;

  fn mul(self, rhs: Matrix) -> Matrix {
    if self.num_cols() == rhs.num_rows() {
      let mut new_mat = Matrix::new(self.num_rows(), rhs.num_cols(), 0.0);
      for i in 0..self.num_rows() {
        for j in 0..rhs.num_cols() {
          let mut sum: f64 = 0.0;
          for k in 0..self.num_cols() {
            sum += self.get(i, k) * rhs.get(k, j);
          }
          new_mat.set(i, j, sum);
        }
      }
      new_mat
    } else {
      panic!("Matrices are not conformable for multiplication.")
    }
  }
}

impl Neg for Matrix {
  type Output = Matrix;

  fn neg(self) -> Matrix {
    let mut m = self.clone();
    for i in 0..self.data.len() {
      m.data[i] = -self.data[i];
    }
    m
  }
}

impl Mul<f64> for Matrix {
  type Output = Matrix;

  fn mul(self, rhs: f64) -> Matrix {
    let mut m = self.clone();
    for i in 0..self.data.len() {
      m.data[i] = rhs * self.data[i];
    }
    m
  }
}

impl PartialEq for Matrix {
  fn eq(&self, other: &Matrix) -> bool {
    if self.num_rows() != other.num_rows() || self.num_cols() != other.num_cols() {
      return false;
    }
    for i in 0..self.data.len() {
      if self.data[i] != other.data[i] {
        return false;
      }
    }
    true
  }
}

impl Eq for Matrix {}

impl Matrix {
  /// Returns the number of rows in the matrix.
  #[inline]
  pub fn num_rows(&self) -> usize{
    self.rows
  }

  /// Returns the number of columns in the matrix.
  #[inline]
  pub fn num_cols(&self) -> usize {
    self.cols
  }

  /// Returns the element at `i`th row and `j`th column of the matrix.
  /// The row, column indexing is 0-based.
  #[inline]
  pub fn get(&self, i: usize, j: usize) -> f64 {
    if i < self.num_rows() && j < self.num_cols() {
      self.data[i * self.num_cols() + j]
    } else {
      panic!(format!("Matrix index ({}, {}) out of bounds.", i, j))
    }
  }

  /// Sets the element at `i`th row and `j`th column of the matrix as `num`.
  /// The row, column indexing is 0-based.
  #[inline]
  pub fn set(&mut self, i: usize, j: usize, num: f64) {
    let (rows, cols) = (self.num_rows(), self.num_cols());
    if i < rows && j < cols {
      self.data[i * cols + j] = num
    } else {
      panic!(format!("Matrix index ({}, {}) out of bounds.", i, j))
    }
  }

  /// Creates a new `Matrix` with dimensions as `rows x cols` with all values
  /// instantiated to `default`.
  pub fn new(rows: usize, cols: usize, default: f64) -> Matrix {
    let mut d: Vec<f64> = Vec::with_capacity(rows * cols);
    for i in 0..rows * cols {
      d.push(default);
    }
    Matrix {
      rows: rows,
      cols: cols,
      data: d
    }
  }

  /// Resizes the dimensions of the matrix with the new dimensions as
  /// `newrows x newcols`.
  pub fn reshape(&mut self, newrows: usize, newcols: usize) {
    if self.rows * self.cols == newrows * newcols {
      self.rows = newrows;
      self.cols = newcols;
    } else {
      panic!("Total number of elements in matrix should be same.")
    }
  }

  /// Returns the matrix as a cloned vector in row major order.
  pub fn get_vec(&self) -> Vec<f64> {
    self.data.clone()
  }

  /// Transposes the matrix.
  pub fn transpose(&mut self) {
    let mut v = self.data.clone();
    for i in 0..self.num_rows() {
      for j in 0..self.num_cols() {
        v[j * self.num_cols() + i] = self.get(i, j);
      }
    }
    self.data = v;
  }

  /// Trace of the matrix.
  pub fn trace(&self) -> f64 {
    if self.num_rows() == self.num_cols() {
      let mut sum: f64 = 0.0;
      for i in 0..self.num_rows() {
        sum += self.get(i, i);
      }
      return sum;
    }
    panic!("The matrix should be a square matrix.")
  }
}

/// Creates a `Matrix` with dimensions `rows x cols` from the elements of the
/// slice `elems`.
pub fn from_elems(rows: usize, cols: usize, elems: &[f64]) -> Matrix {
  let mut m = Matrix {
    rows: rows,
    cols: cols,
    data: Vec::new()
  };
  m.data.extend(elems);
  m
}

/// Creates an identity matrix of dimension `n x n`.
pub fn identity(n: usize) -> Matrix {
  let mut m = Matrix::new(n, n, 0.0);
  for i in 0..n {
    m.set(i, i, 1.0);
  }
  m
}

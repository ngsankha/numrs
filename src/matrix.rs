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

extern crate num;

use self::num::traits::Num;
use std::ops::{Index, Add, Sub, Mul, Neg};

pub struct Matrix<T: Num + Clone + Copy> {
  rows: usize,
  cols: usize,
  data: Vec<T>
}

impl<T: Num + Clone + Copy> Index<usize> for Matrix<T> {
  type Output = [T];

  #[inline]
  fn index<'a>(&'a self, index: usize) -> &'a [T] {
    &self.data[self.rows * index..self.rows * index + self.cols]
  }
}

impl<T: Num + Clone + Copy> Clone for Matrix<T> {
  fn clone(&self) -> Matrix<T> {
    Matrix::<T> {
      rows: self.num_rows(),
      cols: self.num_cols(),
      data: self.data.clone()
    }
  }

  fn clone_from(&mut self, source: &Matrix<T>) {
    self.rows = source.num_rows();
    self.cols = source.num_cols();
    self.data = source.data.clone();
  }
}

impl<T: Num + Clone + Copy> Add<Matrix<T>> for Matrix<T> {
  type Output = Matrix<T>;

  fn add(self, rhs: Matrix<T>) -> Matrix<T> {
    if self.num_rows() == rhs.num_rows() && self.num_cols() == rhs.num_cols() {
      let mut new_mat = Matrix::<T>::new(self.num_rows(), self.num_cols(), T::zero());
      for i in 0..self.data.len() {
        new_mat.data[i] = self.data[i] + rhs.data[i];
      }
      new_mat
    } else {
      panic!("Matrices are not conformable for addition.");
    }
  }
}

impl<T: Num + Clone + Copy> Sub<Matrix<T>> for Matrix<T> {
  type Output = Matrix<T>;

  fn sub(self, rhs: Matrix<T>) -> Matrix<T> {
    if self.num_rows() == rhs.num_rows() && self.num_cols() == rhs.num_cols() {
      let mut new_mat = Matrix::<T>::new(self.num_rows(), self.num_cols(), T::zero());
      for i in 0..self.data.len() {
        new_mat.data[i] = self.data[i] - rhs.data[i];
      }
      new_mat
    } else {
      panic!("Matrices are not conformable for subtraction.");
    }
  }
}

impl<T: Num + Clone + Copy> Mul<Matrix<T>> for Matrix<T> {
  type Output = Matrix<T>;

  fn mul(self, rhs: Matrix<T>) -> Matrix<T> {
    if self.num_cols() == rhs.num_rows() {
      let mut new_mat = Matrix::<T>::new(self.num_rows(), rhs.num_cols(), T::zero());
      for i in 0..self.num_rows() {
        for j in 0..rhs.num_cols() {
          let mut sum: T = T::zero();
          for k in 0..self.num_cols() {
            sum = sum + self.get(i, k) * rhs.get(k, j);
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

impl<T: Num + Clone + Copy + Neg<Output = T>> Neg for Matrix<T> {
  type Output = Matrix<T>;

  fn neg(self) -> Matrix<T> {
    let mut m = self.clone();
    for i in 0..self.data.len() {
      m.data[i] = -self.data[i];
    }
    m
  }
}

impl<T: Num + Clone + Copy> Mul<T> for Matrix<T> {
  type Output = Matrix<T>;

  fn mul(self, rhs: T) -> Matrix<T> {
    let mut m = self.clone();
    for i in 0..self.data.len() {
      m.data[i] = rhs * self.data[i];
    }
    m
  }
}

impl<T: Num + Clone + Copy> PartialEq for Matrix<T> {
  fn eq(&self, other: &Matrix<T>) -> bool {
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

impl<T: Num + Clone + Copy> Eq for Matrix<T> {}

impl<T: Num + Clone + Copy> Matrix<T> {
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
  pub fn get(&self, i: usize, j: usize) -> T {
    if i < self.num_rows() && j < self.num_cols() {
      self.data[i * self.num_cols() + j]
    } else {
      panic!(format!("Matrix index ({}, {}) out of bounds.", i, j))
    }
  }

  /// Sets the element at `i`th row and `j`th column of the matrix as `num`.
  /// The row, column indexing is 0-based.
  #[inline]
  pub fn set(&mut self, i: usize, j: usize, num: T) {
    let (rows, cols) = (self.num_rows(), self.num_cols());
    if i < rows && j < cols {
      self.data[i * cols + j] = num
    } else {
      panic!(format!("Matrix index ({}, {}) out of bounds.", i, j))
    }
  }

  /// Creates a new `Matrix` with dimensions as `rows x cols` with all values
  /// instantiated to `default`.
  pub fn new(rows: usize, cols: usize, default: T) -> Matrix<T> {
    let mut d: Vec<T> = Vec::with_capacity(rows * cols);
    for i in 0..rows * cols {
      d.push(default);
    }
    Matrix::<T> {
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
  pub fn get_vec(&self) -> Vec<T> {
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
  pub fn trace(&self) -> T {
    if self.num_rows() == self.num_cols() {
      let mut sum: T = T::zero();
      for i in 0..self.num_rows() {
        sum = sum + self.get(i, i);
      }
      return sum;
    }
    panic!("The matrix should be a square matrix.")
  }
}

/// Creates a `Matrix` with dimensions `rows x cols` from the elements of the
/// slice `elems`.
pub fn from_elems<T: Num + Clone + Copy>(rows: usize, cols: usize, elems: &[T]) -> Matrix<T> {
  let mut m = Matrix::<T> {
    rows: rows,
    cols: cols,
    data: Vec::new()
  };
  m.data.extend_from_slice(elems);
  m
}

/// Creates an identity matrix of dimension `n x n`.
pub fn identity<T: Num + Clone + Copy>(n: usize) -> Matrix<T> {
  let mut m = Matrix::<T>::new(n, n, T::zero());
  for i in 0..n {
    m.set(i, i, T::one());
  }
  m
}

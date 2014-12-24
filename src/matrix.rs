/// A 2D Matrix type composed of `f64` elements.

pub struct Matrix {
  rows: uint,
  cols: uint,
  data: Vec<f64>
}

impl Index<uint, [f64]> for Matrix {
  #[inline]
  fn index<'a>(&'a self, index: &uint) -> &'a [f64] {
    self.data.as_slice().slice(self.rows * *index, self.rows * *index + self.cols)
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

impl Add<Matrix, Matrix> for Matrix {
  fn add(self, rhs: Matrix) -> Matrix {
    if self.num_rows() == rhs.num_rows() && self.num_cols() == rhs.num_cols() {
      let mut new_mat = Matrix::new(self.num_rows(), self.num_cols(), 0.0);
      for i in range(0, self.data.len()) {
        new_mat.data[i] = self.data[i] + rhs.data[i];
      }
      new_mat
    } else {
      panic!("Matrices are not conformable for addition.");
    }
  }
}

impl Sub<Matrix, Matrix> for Matrix {
  fn sub(self, rhs: Matrix) -> Matrix {
    if self.num_rows() == rhs.num_rows() && self.num_cols() == rhs.num_cols() {
      let mut new_mat = Matrix::new(self.num_rows(), self.num_cols(), 0.0);
      for i in range(0, self.data.len()) {
        new_mat.data[i] = self.data[i] - rhs.data[i];
      }
      new_mat
    } else {
      panic!("Matrices are not conformable for subtraction.");
    }
  }
}

impl Mul<Matrix, Matrix> for Matrix {
  fn mul(self, rhs: Matrix) -> Matrix {
    if self.num_cols() == rhs.num_rows() {
      let mut new_mat = Matrix::new(self.num_rows(), rhs.num_cols(), 0.0);
      for i in range(0, self.num_rows()) {
        for j in range(0, rhs.num_cols()) {
          let mut sum: f64 = 0.0;
          for k in range(0, self.num_cols()) {
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

impl Neg<Matrix> for Matrix {
  fn neg(self) -> Matrix {
    let mut m = self.clone();
    for i in range(0, self.data.len()) {
      m.data[i] = -self.data[i];
    }
    m
  }
}

impl Mul<f64, Matrix> for Matrix {
  fn mul(self, rhs: f64) -> Matrix {
    let mut m = self.clone();
    for i in range(0, self.data.len()) {
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
    for i in range(0, self.data.len()) {
      if self.data[i] != other.data[i] {
        return false;
      }
    }
    true
  }
}

impl Eq for Matrix {}

impl Matrix {
  #[inline]
  pub fn num_rows(&self) -> uint{
    self.rows
  }

  #[inline]
  pub fn num_cols(&self) -> uint {
    self.cols
  }

  #[inline]
  pub fn get(&self, i: uint, j: uint) -> f64 {
    if i < self.num_rows() && j < self.num_cols() {
      self.data[i * self.num_cols() + j]
    } else {
      panic!(format!("Matrix index {} out of bounds.", (i, j)))
    }
  }

  #[inline]
  pub fn set(&mut self, i: uint, j: uint, num: f64) {
    if i < self.num_rows() && j < self.num_cols() {
      self.data.as_mut_slice()[i * self.num_cols() + j] = num
    } else {
      panic!(format!("Matrix index {} out of bounds.", (i, j)))
    }
  }

  pub fn new(rows: uint, cols: uint, default: f64) -> Matrix {
    Matrix {
      rows: rows,
      cols: cols,
      data: Vec::from_elem(rows * cols, default)
    }
  }

  pub fn from_elems(rows: uint, cols: uint, elems: &[f64]) -> Matrix {
    let mut m = Matrix {
      rows: rows,
      cols: cols,
      data: Vec::new()
    };
    m.data.push_all(elems);
    m
  }

  pub fn identity(n: uint) -> Matrix {
    let mut m = Matrix {
      rows: n,
      cols: n,
      data: Vec::from_elem(n * n, 0.0)
    };
    for i in range(0, n) {
      m.set(i, i, 1.0);
    }
    m
  }

  pub fn reshape(&mut self, newrows: uint, newcols: uint) {
    if self.rows * self.cols == newrows * newcols {
      self.rows = newrows;
      self.cols = newcols;
    } else {
      panic!("Total number of elements in matrix should be same.")
    }
  }

  pub fn get_vec(&self) -> Vec<f64> {
    self.data.clone()
  }

  pub fn transpose(&mut self) {
    let mut v = self.data.clone();
    for i in range(0, self.num_rows()) {
      for j in range(0, self.num_cols()) {
        v[j * self.num_cols() + i] = self.get(i, j);
      }
    }
    self.data = v;
  }
}

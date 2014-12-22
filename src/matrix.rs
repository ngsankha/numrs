pub struct Matrix {
  rows: uint,
  cols: uint,
  data: Vec<f64>
}

impl Index<uint, [f64]> for Matrix {
  fn index<'a>(&'a self, index: &uint) -> &'a [f64] {
    self.data.as_slice().slice(self.rows * *index, self.rows * *index + self.cols)
  }
}

impl Matrix {
  #[inline(always)]
  pub fn num_rows(&self) -> uint{
    self.rows
  }

  #[inline(always)]
  pub fn num_cols(&self) -> uint {
    self.cols
  }

  #[inline(always)]
  pub fn get(&self, i: uint, j: uint) -> f64 {
    if i < self.num_rows() && j < self.num_cols() {
      self.data[i * self.num_cols() + j]
    } else {
      panic!(format!("Matrix index {} out of bounds.", (i, j)))
    }
  }

  #[inline(always)]
  pub fn set(&mut self, i: uint, j: uint, num: f64) {
    if i < self.num_rows() && j < self.num_cols() {
      self.data.as_mut_slice()[i * self.num_cols() + j] = num
    } else {
      panic!(format!("Matrix index {} out of bounds.", (i, j)))
    }
  }

  pub fn add(&self, m: Matrix) -> Matrix {
    if self.num_rows() == m.num_rows() && self.num_cols() == self.num_cols() {
      let mut new_mat = Matrix::new(self.num_rows(), self.num_cols(), 0.0);
      for i in range(0, self.num_rows()) {
        for j in range(0, self.num_cols()) {
          new_mat.set(i, j, self.get(i, j) + m.get(i, j));
        }
      }
      new_mat
    } else {
      panic!("Matrices are not conformable for addition.")
    }
  }

  pub fn multiply(&self, m: Matrix) -> Matrix {
    if self.num_cols() == m.num_rows() {
      let mut new_mat = Matrix::new(self.num_rows(), m.num_cols(), 0.0);
      for i in range(0, self.num_rows()) {
        for j in range(0, m.num_cols()) {
          let mut sum: f64 = 0.0;
          for k in range(0, self.num_cols()) {
            sum += self.get(i, k) * m.get(k, j);
          }
          new_mat.set(i, j, sum);
        }
      }
      new_mat
    } else {
      panic!("Matrices are not conformable for multiplication.")
    }
  }

  pub fn scalar_multiply(&mut self, m: f64) {
    for i in range(0, self.num_rows()) {
      for j in range(0, self.num_cols()) {
        let tmp: f64 = m * self.get(i, j);
        self.set(i, j, tmp);
      }
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
}

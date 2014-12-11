pub struct Matrix {
  rows: uint,
  cols: uint,
  data: Vec<f64>
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

  pub fn new(rows: uint, cols: uint, default: f64) -> Matrix {
    Matrix {
      rows: rows,
      cols: cols,
      data: Vec::from_elem(rows * cols, default)
    }
  }
}

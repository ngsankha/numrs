use matrix::Matrix;

pub fn gauss_seidel(a: Matrix, b: Matrix, mut x: Matrix, iter: uint) -> Matrix {
  for i in range(0, iter) {
    for j in range(0, x.num_rows()) {
      let mut sum: f64 = b.get(j, 0);
      for k in range(0, x.num_rows()) {
        if j != k {
          sum -= a.get(j, k) * x.get(k, 0);
        }
      }
      x.set(j, 0, sum / a.get(j, j));
    }
  }
  x
}

mod matrix;

fn main() {
  let mut m = matrix::Matrix::new(2, 2, 0.0);
  m.set(0, 0, 3.142);
  for i in range(0, m.num_rows()) {
    for j in range(0, m.num_cols()) {
      println!("{}", m.get(i, j));
    }
  }
}

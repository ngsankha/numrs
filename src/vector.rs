extern crate num;

use self::num::traits::Float;
use std::ops::{Index};

pub struct Vector<T: Float> {
  data: Vec<T>
}

impl<T: Float> Vector<T> {
  pub fn new(elems: &[T]) -> Vector<T> {
    let mut v = Vector::<T> { data: Vec::new() };
    v.data.extend(elems);
    v
  }
}

impl<T: Float> Index<usize> for Vector<T> {
  type Output = T;

  #[inline]
  fn index<'a>(&'a self, index: usize) -> &'a T {
    &self.data[index]
  }
}

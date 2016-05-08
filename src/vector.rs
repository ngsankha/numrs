use std::ops::{Index, IndexMut, Add, Sub, Mul, Neg};
pub use common::Number;

pub struct Vector<T: Number> {
  pub data: Vec<T>
}

impl<T: Number> Index<usize> for Vector<T> {
  type Output = T;

  #[inline]
  fn index<'a>(&'a self, index: usize) -> &'a T {
    &self.data[index]
  }
}

impl<T: Number> IndexMut<usize> for Vector<T> {
  #[inline]
  fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
    &mut self.data[index]
  }
}

impl<T: Number> Clone for Vector<T> {
  fn clone(&self) -> Vector<T> {
    Vector::<T> {
      data: self.data.clone()
    }
  }

  fn clone_from(&mut self, source: &Vector<T>) {
    self.data = source.data.clone();
  }
}

impl<T: Number> Add<Vector<T>> for Vector<T> {
  type Output = Vector<T>;

  fn add(self, rhs: Vector<T>) -> Vector<T> {
    if self.len() == rhs.len() {
      let mut new_vec = Vector::<T>::new(self.len(), T::zero());
      for i in 0..self.len() {
        new_vec.data[i] = self.data[i] + rhs.data[i];
      }
      new_vec
    } else {
      panic!("Vectors are not conformable for addition.");
    }
  }
}

impl<T: Number> Sub<Vector<T>> for Vector<T> {
  type Output = Vector<T>;

  fn sub(self, rhs: Vector<T>) -> Vector<T> {
    if self.len() == rhs.len() {
      let mut new_vec = Vector::<T>::new(self.len(), T::zero());
      for i in 0..self.len() {
        new_vec.data[i] = self.data[i] - rhs.data[i];
      }
      new_vec
    } else {
      panic!("Vectors are not conformable for subtraction.");
    }
  }
}

impl<T: Number> Mul<Vector<T>> for Vector<T> {
  type Output = Vector<T>;

  fn mul(self, rhs: Vector<T>) -> Vector<T> {
    if self.len() == rhs.len() {
      let mut new_vec = Vector::<T>::new(self.len(), T::zero());
      for i in 0..self.len() {
        new_vec.data[i] = self.data[i] * rhs.data[i];
      }
      new_vec
    } else {
      panic!("Vectors are not conformable for multiplication.")
    }
  }
}

impl<T: Number + Neg<Output = T>> Neg for Vector<T> {
  type Output = Vector<T>;

  fn neg(self) -> Vector<T> {
    let mut v = self.clone();
    for i in 0..self.len() {
      v.data[i] = -self.data[i];
    }
    v
  }
}

impl<T: Number> Mul<T> for Vector<T> {
  type Output = Vector<T>;

  fn mul(self, rhs: T) -> Vector<T> {
    let mut v = self.clone();
    for i in 0..self.len() {
      v.data[i] = rhs * self.data[i];
    }
    v
  }
}

impl<T: Number> PartialEq for Vector<T> {
  fn eq(&self, other: &Vector<T>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    for i in 0..self.len() {
      if self.data[i] != other.data[i] {
        return false;
      }
    }
    true
  }
}

impl<T: Number> Eq for Vector<T> {}

impl<T: Number> Vector<T> {
  #[inline]
  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn new(length: usize, default: T) -> Vector<T> {
    let d = vec![default; length];
    Vector::<T> {
      data: d
    }
  }
}

pub fn from_elems<T: Number>(elems: &[T]) -> Vector<T> {
  let mut v = Vector::<T> {
    data: Vec::with_capacity(elems.len())
  };
  v.data.extend_from_slice(elems);
  v
}

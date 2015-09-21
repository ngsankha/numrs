extern crate numrs;
use numrs::vector::Vector;

#[test]
fn test_basic_vector() {
  let arr = [1.0, 2.0, 3.0];
  let v = Vector::<f32>::new(&arr);
  assert_eq!(v[0], 1.0);
  assert_eq!(v[1], 2.0);
  assert_eq!(v[2], 3.0);
  assert_eq!(v.len(), 3);
}

#[test]
fn test_vector_clone() {
  let arr = [1.0, 2.0, 3.0];
  let v1 = Vector::<f32>::new(&arr);
  let v2 = v1.clone();
  assert!(v1 == v2);
}

#[test]
fn test_vector_add() {
  let arr = [1.0, 2.0, 3.0, 4.0, 5.0];
  let v1 = Vector::<f32>::new(&arr);
  let v2 = Vector::<f32>::new(&arr);
  let res = (v1 + v2).unwrap();
  assert_eq!(res[0], 2.0);
  assert_eq!(res[1], 4.0);
  assert_eq!(res[2], 6.0);
  assert_eq!(res[3], 8.0);
  assert_eq!(res[4], 10.0);
  assert_eq!(res.len(), 5);
}

#[test]
fn test_vector_sub() {
  let arr1 = [1.0, 2.0, 3.0, 4.0, 5.0];
  let arr2 = [1.0, 1.0, 1.0, 1.0, 1.0];
  let v1 = Vector::<f32>::new(&arr1);
  let v2 = Vector::<f32>::new(&arr2);
  let res = (v1 - v2).unwrap();
  assert_eq!(res[0], 0.0);
  assert_eq!(res[1], 1.0);
  assert_eq!(res[2], 2.0);
  assert_eq!(res[3], 3.0);
  assert_eq!(res[4], 4.0);
  assert_eq!(res.len(), 5);
}

#[test]
fn test_vector_mul() {
  let arr = [1.0, 2.0, 3.0, 4.0, 5.0];
  let v1 = Vector::<f32>::new(&arr);
  let v2 = Vector::<f32>::new(&arr);
  let res = (v1 * v2).unwrap();
  assert_eq!(res[0], 1.0);
  assert_eq!(res[1], 4.0);
  assert_eq!(res[2], 9.0);
  assert_eq!(res[3], 16.0);
  assert_eq!(res[4], 25.0);
  assert_eq!(res.len(), 5);
}

#[test]
fn test_vector_div() {
  let arr1 = [2.0, 4.0, 6.0, 8.0, 10.0];
  let arr2 = [2.0, 2.0, 2.0, 2.0, 2.0];
  let v1 = Vector::<f32>::new(&arr1);
  let v2 = Vector::<f32>::new(&arr2);
  let res = (v1 / v2).unwrap();
  assert_eq!(res[0], 1.0);
  assert_eq!(res[1], 2.0);
  assert_eq!(res[2], 3.0);
  assert_eq!(res[3], 4.0);
  assert_eq!(res[4], 5.0);
  assert_eq!(res.len(), 5);
}

#[test]
fn test_vector_neg() {
  let arr = [1.0, 2.0, 3.0, 4.0, 5.0];
  let v = Vector::<f32>::new(&arr);
  let res = -v.clone();
  assert_eq!(res[0], -v[0]);
  assert_eq!(res[1], -v[1]);
  assert_eq!(res[2], -v[2]);
  assert_eq!(res[3], -v[3]);
  assert_eq!(res[4], -v[4]);
  assert_eq!(res.len(), 5);
}

#[test]
fn test_basic_vector_f64() {
  let arr = [1.0, 2.0, 3.0];
  let v = Vector::<f64>::new(&arr);
  assert_eq!(v[0], 1.0);
  assert_eq!(v[1], 2.0);
  assert_eq!(v[2], 3.0);
  assert_eq!(v.len(), 3);
}

#[test]
fn test_vector_clone_f64() {
  let arr = [1.0, 2.0, 3.0];
  let v1 = Vector::<f64>::new(&arr);
  let v2 = v1.clone();
  assert!(v1 == v2);
}

#[test]
fn test_vector_add_f64() {
  let arr = [1.0, 2.0, 3.0, 4.0, 5.0];
  let v1 = Vector::<f64>::new(&arr);
  let v2 = Vector::<f64>::new(&arr);
  let res = (v1 + v2).unwrap();
  assert_eq!(res[0], 2.0);
  assert_eq!(res[1], 4.0);
  assert_eq!(res[2], 6.0);
  assert_eq!(res[3], 8.0);
  assert_eq!(res[4], 10.0);
  assert_eq!(res.len(), 5);
}

#[test]
fn test_vector_sub_f64() {
  let arr1 = [1.0, 2.0, 3.0, 4.0, 5.0];
  let arr2 = [1.0, 1.0, 1.0, 1.0, 1.0];
  let v1 = Vector::<f64>::new(&arr1);
  let v2 = Vector::<f64>::new(&arr2);
  let res = (v1 - v2).unwrap();
  assert_eq!(res[0], 0.0);
  assert_eq!(res[1], 1.0);
  assert_eq!(res[2], 2.0);
  assert_eq!(res[3], 3.0);
  assert_eq!(res[4], 4.0);
  assert_eq!(res.len(), 5);
}

#[test]
fn test_vector_mul_f64() {
  let arr = [1.0, 2.0, 3.0, 4.0, 5.0];
  let v1 = Vector::<f64>::new(&arr);
  let v2 = Vector::<f64>::new(&arr);
  let res = (v1 * v2).unwrap();
  assert_eq!(res[0], 1.0);
  assert_eq!(res[1], 4.0);
  assert_eq!(res[2], 9.0);
  assert_eq!(res[3], 16.0);
  assert_eq!(res[4], 25.0);
  assert_eq!(res.len(), 5);
}

#[test]
fn test_vector_div_f64() {
  let arr1 = [2.0, 4.0, 6.0, 8.0, 10.0];
  let arr2 = [2.0, 2.0, 2.0, 2.0, 2.0];
  let v1 = Vector::<f64>::new(&arr1);
  let v2 = Vector::<f64>::new(&arr2);
  let res = (v1 / v2).unwrap();
  assert_eq!(res[0], 1.0);
  assert_eq!(res[1], 2.0);
  assert_eq!(res[2], 3.0);
  assert_eq!(res[3], 4.0);
  assert_eq!(res[4], 5.0);
  assert_eq!(res.len(), 5);
}

#[test]
fn test_vector_neg_f64() {
  let arr = [1.0, 2.0, 3.0, 4.0, 5.0];
  let v = Vector::<f64>::new(&arr);
  let res = -v.clone();
  assert_eq!(res[0], -v[0]);
  assert_eq!(res[1], -v[1]);
  assert_eq!(res[2], -v[2]);
  assert_eq!(res[3], -v[3]);
  assert_eq!(res[4], -v[4]);
  assert_eq!(res.len(), 5);
}

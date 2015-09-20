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
fn test_vector_add() {
  let arr = [1.0, 2.0, 3.0, 4.0, 5.0];
  let v1 = Vector::<f32>::new(&arr);
  let v2 = Vector::<f32>::new(&arr);
  let res = v1 + v2;
  assert_eq!(res[0], 2.0);
  assert_eq!(res[1], 4.0);
  assert_eq!(res[2], 6.0);
  assert_eq!(res[3], 8.0);
  assert_eq!(res[4], 10.0);
  assert_eq!(res.len(), 5);
}

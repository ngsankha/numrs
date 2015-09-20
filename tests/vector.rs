extern crate numrs;
use numrs::vector::Vector;

#[test]
fn test_basic_vector() {
  let arr = [1.0, 2.0, 3.0];
  let v = Vector::<f32>::new(&arr);
  assert_eq!(v[0], 1.0);
  assert_eq!(v[1], 2.0);
  assert_eq!(v[2], 3.0);
}

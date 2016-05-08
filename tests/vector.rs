extern crate numrs;
use numrs::vector;
use numrs::vector::Vector;

#[test]
fn test_basic_vector() {
    let mut v = Vector::new(4, 0.0);
    assert_eq!(v.len(), 4);
    v[3] = 1.0;
    assert_eq!(v[0], 0.0);
    assert_eq!(v[1], 0.0);
    assert_eq!(v[2], 0.0);
    assert_eq!(v[3], 1.0);
}

#[test]
fn test_vector_add() {
    let elems = [1.0, 2.0, 3.0, 4.0];
    let v1 = vector::from_elems(&elems);
    let v2 = vector::from_elems(&elems);
    let res = v1 + v2;

    assert_eq!(res[0], 2.0);
    assert_eq!(res[1], 4.0);
    assert_eq!(res[2], 6.0);
    assert_eq!(res[3], 8.0);
}

#[test]
fn test_vector_multiply() {
    let elems = [1.0, 2.0, 3.0, 4.0];
    let v1 = vector::from_elems(&elems);
    let v2 = vector::from_elems(&elems);
    let res = v1 * v2;

    assert_eq!(res[0], 1.0);
    assert_eq!(res[1], 4.0);
    assert_eq!(res[2], 9.0);
    assert_eq!(res[3], 16.0);
}

#[test]
fn test_scalar_multiply() {
    let elems = [1.0, 2.0, 3.0, 4.0];
    let v = vector::from_elems(&elems);

    let mut v_ = -v.clone();
    assert_eq!(v_[0], -1.0);
    assert_eq!(v_[1], -2.0);
    assert_eq!(v_[2], -3.0);
    assert_eq!(v_[3], -4.0);

    v_ = v.clone() * 2.0;
    assert_eq!(v_[0], 2.0);
    assert_eq!(v_[1], 4.0);
    assert_eq!(v_[2], 6.0);
    assert_eq!(v_[3], 8.0);
}

#[test]
fn test_vector_clone() {
    let elems = [16.0, 3.0, 7.0, -11.0];
    let v1 = vector::from_elems(&elems);
    let v2 = v1.clone();
    assert_eq!(v2[0], 16.0);
    assert_eq!(v2[1], 3.0);
    assert_eq!(v2[2], 7.0);
    assert_eq!(v2[3], -11.0);
}

#[test]
fn test_vector_eq() {
    let elem1 = [1.0, 2.0, 3.0, 4.0];
    let v1 = vector::from_elems(&elem1);
    let elem2 = [1.0, 3.0, 3.0, 4.0];
    let v2 = vector::from_elems(&elem2);
    assert!(v1 != v2)
}

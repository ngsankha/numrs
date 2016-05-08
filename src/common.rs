extern crate num;

use self::num::traits::Num;

/// A generic number trait for primitive integers and floating point numbers
pub trait Number: Num + Clone + Copy {}

impl Number for f64 {}
impl Number for f32 {}
impl Number for i64 {}
impl Number for i32 {}
impl Number for i16 {}
impl Number for i8 {}
impl Number for u64 {}
impl Number for u32 {}
impl Number for u16 {}
impl Number for u8 {}
impl Number for usize {}

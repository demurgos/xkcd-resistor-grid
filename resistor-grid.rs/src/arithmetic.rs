use std::fmt;
use std::ops;
use std::cmp;
use std::default;

pub trait Arithmetic: default::Default + fmt::Display + Clone + ops::AddAssign + ops::SubAssign + ops::Mul<Output=Self> + cmp::PartialEq<i32> {}

impl<T: default::Default + fmt::Display + Clone + ops::AddAssign + ops::SubAssign + ops::Mul<Output=T> + cmp::PartialEq<i32>> Arithmetic for T {}

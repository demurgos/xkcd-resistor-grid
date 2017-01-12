use std::fmt;
use std::ops;
use std::cmp;
use std::default;

pub trait Arithmetic: default::Default + fmt::Display + Clone + ops::AddAssign + ops::Sub<Output=Self> + ops::SubAssign + ops::Mul<Output=Self> + ops::DivAssign + cmp::PartialEq<Self> {}

impl<T: default::Default + fmt::Display + Clone + ops::AddAssign + ops::Sub<Output=T> + ops::SubAssign + ops::Mul<Output=T> + ops::DivAssign + cmp::PartialEq<T>> Arithmetic for T {}

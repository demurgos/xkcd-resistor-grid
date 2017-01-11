use std::fmt;
use std::ops;
use std::cmp;
use std::default;

pub trait Arithmetic: default::Default + fmt::Display + Clone + ops::Add<Output = Self> + ops::AddAssign + ops::Mul<Output = Self> + cmp::PartialEq<i32> {}

impl<T: default::Default + fmt::Display + Clone + ops::Add<Output = T> + ops::AddAssign + ops::Mul<Output = T> + cmp::PartialEq<i32>> Arithmetic for T {}

#[derive(Clone)]
pub struct Matrix<T: Arithmetic> {
  coefficients: Vec<T>,
  rows: usize,
  cols: usize
}

impl<T: Arithmetic> Matrix<T> {
  pub fn is_square(&self) -> bool {
    self.rows == self.cols
  }
}

impl<T: Arithmetic> ops::Add<Matrix<T>> for Matrix<T> {
  type Output = Matrix<T>;

  fn add(self, rhs: Matrix<T>) -> Matrix<T> {
    &self + &rhs
  }
}

impl<'a, 'b, T: Arithmetic> ops::Add<&'b Matrix<T>> for &'a Matrix<T> {
  type Output = Matrix<T>;

  fn add(self, rhs: &'b Matrix<T>) -> Matrix<T> {
    if self.rows != rhs.rows || self.cols != rhs.cols {
      panic!("Incompatible matrix sizes for addition");
    }

    let mut coefficients: Vec<T> = self.coefficients.clone();
    for (index, coeff) in rhs.coefficients.iter().enumerate() {
      coefficients[index] += coeff.clone();
    }

    Matrix::<T> {
      coefficients:  coefficients,
      cols: self.cols,
      rows: self.rows
    }
  }
}

//impl<T: Arithmetic> ops::AddAssign<Matrix<T>> for Matrix<T> {
//  fn add_assign(&mut self, rhs: Matrix<T>) {
//    self.add_assign(&rhs)
//  }
//}
//
//impl<'a, T: Arithmetic> ops::AddAssign<&'a Matrix<T>> for Matrix<T> {
//  fn add_assign(&mut self, rhs: &'a Matrix<T>) {
//    if self.rows != rhs.rows || self.colrs != rhs.cols {
//      panic!("Incompatible matrix sizes for addition");
//    }
//    self.coefficients += rhs.coefficients;
//    self
//  }
//}
//
//impl<T: Arithmetic> ops::Sub<Matrix<T>> for Matrix<T> {
//  type Output = Matrix<T>;
//
//  fn sub(self, rhs: Matrix<T>) -> Matrix<T> {
//    &self - &rhs
//  }
//}
//
//impl<'a, 'b, T: Arithmetic> ops::Sub<&'b Matrix<T>> for &'a Matrix<T> {
//  type Output = Matrix<T>;
//
//  fn sub(self, rhs: &'b Matrix<T>) -> Matrix<T> {
//    if self.rows != rhs.rows || self.colrs != rhs.cols {
//      panic!("Incompatible matrix sizes for substraction");
//    }
//
//    Matrix::<T> {
//      coefficients: self.coefficients.clone() - rhs.coefficients.clone(),
//      cols: self.cols,
//      rows: self.rows
//    }
//  }
//}

impl<T: Arithmetic> fmt::Display for Matrix<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // let indent = 8;
    try! {write!(f, "(")}
    for (index, coeff) in self.coefficients.iter().enumerate() {
      let col = index % self.cols;
      let row = (index - col) / self.rows;
      if col == 0 {
        if row > 0 {
          try! {write!(f, " ")}
        }
        try! {write!(f, "(")}
      } else {
        try! {write!(f, ", ")}
      }
      try! {write!(f, "{}", coeff)}
      if col == self.cols - 1 {
        try! {write!(f, ")")}
        if row < self.rows - 1 {
          try! {write!(f, "\n")}
        }
      }
    }
    write!(f, ")")
  }
}

#[cfg(test)]
mod tests {
  use super::Matrix;

  #[test]
  fn test_fmt2() {
    let m1: Matrix<i32> = Matrix::<i32> {coefficients: vec!(1i32, 2i32, 3i32, 4i32), rows: 2, cols: 2};
    assert_eq!(m1.to_string(), "((1, 2)\n (3, 4))");
  }
}

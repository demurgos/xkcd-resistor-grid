use arithmetic::Arithmetic;

use std::fmt;
use std::ops;

#[derive(Clone)]
pub struct Matrix<T: Arithmetic> {
  rows: usize,
  cols: usize,
  data: Vec<T>
}

impl<T: Arithmetic> ops::Index<[usize; 2]> for Matrix<T> {
  type Output = T;

  fn index(&self, index: [usize; 2]) -> &T {
    assert!(index[0] < self.rows, "Row index is greater than row dimension.");
    assert!(index[1] < self.cols, "Column index is greater than column dimension.");

    return &self.data[index[0] * self.cols + index[1]];
  }
}

impl<T: Arithmetic> ops::IndexMut<[usize; 2]> for Matrix<T> {
  fn index_mut(&mut self, index: [usize; 2]) -> &mut T {
    assert!(index[0] < self.rows, "Row index is greater than row dimension.");
    assert!(index[1] < self.cols, "Column index is greater than column dimension.");

    return &mut self.data[index[0] * self.cols + index[1]];
  }
}

impl<T: Arithmetic> Matrix<T> {
  pub fn is_square(&self) -> bool {
    self.rows == self.cols
  }

  pub fn det(&self) -> T {
    assert!(self.is_square(), "Matrix must be a square to compute determinant");

    let mut mat: Matrix<T> = self.clone();
    let size: usize = self.rows;

    for i in 0..(size - 1) {
      for j in (i + 1)..size {
        for k in (i + 1)..size {
          mat[[j, k]] = (mat[[j, k]].clone() * mat[[i, i]].clone()) - (mat[[j, i]].clone() * mat[[i, k]].clone());
          if i > 0 {
            mat[[j, k]] /= mat[[i - 1, i - 1]].clone();
          }
        }
      }
      // println!("{}", i);
      if i > 0 {
        for j in 0..size {
          mat[[j, i - 1]] = T::default();
          mat[[i - 1, j]] = T::default();
        }
      }
    }
    mat[[size - 1, size - 1]].clone()
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
    assert!(self.rows == rhs.rows && self.cols == rhs.cols, "Incompatible matrix sizes for addition");

    let mut coefficients: Vec<T> = self.data.clone();
    for (index, coeff) in rhs.data.iter().enumerate() {
      coefficients[index] += coeff.clone();
    }

    Matrix::<T> {
      data:  coefficients,
      cols: self.cols,
      rows: self.rows
    }
  }
}

impl<T: Arithmetic> ops::AddAssign<Matrix<T>> for Matrix<T> {
  fn add_assign(&mut self, rhs: Matrix<T>) {
    self.add_assign(&rhs)
  }
}

impl<'a, T: Arithmetic> ops::AddAssign<&'a Matrix<T>> for Matrix<T> {
  fn add_assign(&mut self, rhs: &'a Matrix<T>) {
    if self.rows != rhs.rows || self.cols != rhs.cols {
      assert!(self.rows == rhs.rows && self.cols == rhs.cols, "Incompatible matrix sizes for addition");
    }
    for (index, coeff) in rhs.data.iter().enumerate() {
      self.data[index] += coeff.clone();
    }
  }
}

impl<T: Arithmetic> ops::Sub<Matrix<T>> for Matrix<T> {
  type Output = Matrix<T>;

  fn sub(self, rhs: Matrix<T>) -> Matrix<T> {
    &self - &rhs
  }
}

impl<'a, 'b, T: Arithmetic> ops::Sub<&'b Matrix<T>> for &'a Matrix<T> {
  type Output = Matrix<T>;

  fn sub(self, rhs: &'b Matrix<T>) -> Matrix<T> {
    assert!(self.rows == rhs.rows && self.cols == rhs.cols, "Incompatible matrix sizes for subtraction");

    let mut coefficients: Vec<T> = self.data.clone();
    for (index, coeff) in rhs.data.iter().enumerate() {
      coefficients[index] -= coeff.clone();
    }

    Matrix::<T> {
      data:  coefficients,
      cols: self.cols,
      rows: self.rows
    }
  }
}

impl<T: Arithmetic> fmt::Display for Matrix<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // let indent = 8;
    try! {write!(f, "(")}
    for (index, coeff) in self.data.iter().enumerate() {
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

pub fn foo <T: Arithmetic> (a: T) {
  let b = Matrix::<T>{rows: 2, cols: 2, data: vec!(a.clone(), a.clone(), a.clone(), a.clone())};
  println!("{}", b);
}

#[cfg(test)]
mod tests {
  use super::Matrix;

  #[test]
  fn test_fmt() {
    let m1: Matrix<i32> = Matrix::<i32> {rows: 2, cols: 2, data: vec!(1i32, 2i32, 3i32, 4i32)};
    assert_eq!(m1.to_string(), "((1, 2)\n (3, 4))");
    assert!(m1.is_square());
  }

  #[test]
  fn test_index() {
    let m1: Matrix<i32> = Matrix::<i32> {rows: 2, cols: 2, data: vec!(1i32, 2i32, 3i32, 4i32)};
    assert_eq!(m1[[0, 0]], 1i32);
    assert_eq!(m1[[0, 1]], 2i32);
    assert_eq!(m1[[1, 0]], 3i32);
    assert_eq!(m1[[1, 1]], 4i32);
  }

  #[test]
  fn test_add() {
    let m1: Matrix<i32> = Matrix::<i32> {rows: 2, cols: 2, data: vec!(1i32, 2i32, 3i32, 4i32)};
    let m2: Matrix<i32> = Matrix::<i32> {rows: 2, cols: 2, data: vec!(1i32, 2i32, 3i32, 4i32)};
    let m3 = m1 + m2;
    assert_eq!(m3.to_string(), "((2, 4)\n (6, 8))");
  }

  #[test]
  fn test_det() {
    let m1: Matrix<f32> = Matrix::<f32> {rows: 2, cols: 2, data: vec!(1., 2., 3., 4.)};
    assert_eq!(m1.det(), -2.);
  }
}

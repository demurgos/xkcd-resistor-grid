use std::fmt;
use std::ops;
use std::cmp;
use std::default;

pub trait Arithmetic: default::Default + fmt::Display + Clone + ops::AddAssign + ops::Mul<Output=Self> + cmp::PartialEq<i32> {}

impl<T: default::Default + fmt::Display + Clone + ops::AddAssign + ops::Mul<Output=T> + cmp::PartialEq<i32>> Arithmetic for T {}

#[derive(Clone)]
pub struct Polynomial<T: Arithmetic> {
  coefficients: Vec<T>
}

#[derive(PartialEq)]
pub enum PolynomialDegree {
  NegativeInfinity,
  FiniteValue(usize)
}

fn drop_trailing_zeros<T: Arithmetic>(coefficients: &mut Vec<T>) {
  while coefficients.len() > 0 && coefficients[coefficients.len() - 1] == Default::default() {
    coefficients.pop();
  }
}

impl<T: Arithmetic> Polynomial<T> {
  pub fn degree(&self) -> PolynomialDegree {
    let len = self.coefficients.len();
    match len {
      0 => PolynomialDegree::NegativeInfinity,
      _ => PolynomialDegree::FiniteValue(len - 1)
    }
  }
}

impl<T: Arithmetic> ops::Add<Polynomial<T>> for Polynomial<T> {
  type Output = Polynomial<T>;

  fn add(self, rhs: Polynomial<T>) -> Polynomial<T> {
    &self + &rhs
  }
}

impl<'a, 'b, T: Arithmetic> ops::Add<&'b Polynomial<T>> for &'a Polynomial<T> {
  type Output = Polynomial<T>;

  fn add(self, rhs: &'b Polynomial<T>) -> Polynomial<T> {
    match rhs.degree() {
      PolynomialDegree::NegativeInfinity => { self.clone() },
      PolynomialDegree::FiniteValue(rhs_deg) => {
        match self.degree() {
          PolynomialDegree::NegativeInfinity => {
            rhs.clone()
          },
          PolynomialDegree::FiniteValue(self_deg) => {
            return if self_deg > rhs_deg {
              let mut result = self.clone();
              result += rhs.clone();
              result
            } else {
              let mut result = rhs.clone();
              result += self.clone();
              result
            }
          }
        }
      }
    }
  }
}

impl<T: Arithmetic> ops::AddAssign<Polynomial<T>> for Polynomial<T> {
  fn add_assign(&mut self, rhs: Polynomial<T>) {
    self.add_assign(&rhs)
  }
}

impl<'a, T: Arithmetic> ops::AddAssign<&'a Polynomial<T>> for Polynomial<T> {
  fn add_assign(&mut self, rhs: &'a Polynomial<T>) {
    match rhs.degree() {
      PolynomialDegree::NegativeInfinity => {},
      PolynomialDegree::FiniteValue(rhs_deg) => {
        match self.degree() {
          PolynomialDegree::NegativeInfinity => {
            self.coefficients = rhs.coefficients.clone();
          },
          PolynomialDegree::FiniteValue(self_deg) => {
            if self_deg > rhs_deg {
              for (degree, coeff) in rhs.coefficients.iter().enumerate() {
                self.coefficients[degree] += coeff.clone()
              }
            } else {
              let mut tmp_coefficients = rhs.coefficients.clone();
              for (degree, coeff) in self.coefficients.iter().enumerate() {
                tmp_coefficients[degree] += coeff.clone()
              }
              drop_trailing_zeros(&mut tmp_coefficients);
              self.coefficients = tmp_coefficients
            }
          }
        }
      }
    }
  }
}

impl<T: Arithmetic> ops::Mul<Polynomial<T>> for Polynomial<T> {
  type Output = Polynomial<T>;

  fn mul(self, rhs: Polynomial<T>) -> Polynomial<T> {
    &self * &rhs
  }
}

impl<'a, 'b, T: Arithmetic> ops::Mul<&'b Polynomial<T>> for &'a Polynomial<T> {
  type Output = Polynomial<T>;

  fn mul(self, rhs: &'b Polynomial<T>) -> Polynomial<T> {
    match rhs.degree() {
      PolynomialDegree::NegativeInfinity => { rhs.clone() },
      PolynomialDegree::FiniteValue(rhs_deg) => {
        match self.degree() {
          PolynomialDegree::NegativeInfinity => {
            self.clone()
          },
          PolynomialDegree::FiniteValue(self_deg) => {
            let mut tmp_coefficients: Vec<T> = vec![Default::default(); rhs_deg + self_deg + 2];
            for (self_index, self_value) in self.coefficients.iter().enumerate() {
              for (rhs_index, rhs_value) in rhs.coefficients.iter().enumerate() {
                tmp_coefficients[self_index + rhs_index] += self_value.clone() * rhs_value.clone();
              }
            }
            drop_trailing_zeros(&mut tmp_coefficients);
            Polynomial::<T> { coefficients: tmp_coefficients }
          }
        }
      }
    }
  }
}

impl<T: Arithmetic> fmt::Display for Polynomial<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.degree() {
      PolynomialDegree::NegativeInfinity => write!(f, "P[(0)]"),
      PolynomialDegree::FiniteValue(polynomial_degree) => {
        try! {write!(f, "P[")};
        for (degree, coeff) in self.coefficients.iter().enumerate().rev() {
          if *coeff == 0i32 {
            continue;
          }

          if (degree as usize) < polynomial_degree {
            try! {write!(f, " + ")};
          }
          try! {write!(f, "({})", coeff)};
          match degree {
            0 => {},
            1 => try! {write!(f, "x")},
            _ => try! {write!(f, "x^{}", degree)}
          }
        }
        write!(f, "]")
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Polynomial;

  #[test]
  fn test_fmt() {
    let poly = Polynomial::<i32> { coefficients: vec![1, 2, 3, 4] };

    assert_eq!(poly.to_string(), "P[(4)x^3 + (3)x^2 + (2)x + (1)]");
  }

  #[test]
  fn test_add() {
    let poly1 = Polynomial::<i32> { coefficients: vec![1, 10] };
    let poly2 = Polynomial::<i32> { coefficients: vec![2, 20] };
    let poly3 = Polynomial::<i32> { coefficients: vec![0, -20] };

    let sum1 = &poly1 + &poly2;
    let sum2 = &poly2 + &poly3;

    assert_eq!(poly1.to_string(), "P[(10)x + (1)]");
    assert_eq!(poly2.to_string(), "P[(20)x + (2)]");
    assert_eq!(poly3.to_string(), "P[(-20)x]");
    assert_eq!(sum1.to_string(), "P[(30)x + (3)]");
    assert_eq!(sum2.to_string(), "P[(2)]");
  }

  #[test]
  fn test_mul() {
    let poly1 = Polynomial::<i32> { coefficients: vec![1, 10] };
    let poly2 = Polynomial::<i32> { coefficients: vec![2, 20] };
    let poly3 = Polynomial::<i32> { coefficients: vec![0] };

    let mul1 = &poly1 * &poly2;
    let mul2 = &poly1 * &poly3;

    assert_eq!(poly1.to_string(), "P[(10)x + (1)]");
    assert_eq!(poly2.to_string(), "P[(20)x + (2)]");
    assert_eq!(poly3.to_string(), "P[]");
    assert_eq!(mul1.to_string(), "P[(200)x^2 + (40)x + (2)]");
    assert_eq!(mul2.to_string(), "P[(0)]");
  }

  #[test]
  fn test_add_assign() {
    let mut poly1 = Polynomial::<i32> { coefficients: vec![2, 20] };
    let poly2 = Polynomial::<i32> { coefficients: vec![1, -10] };
    poly1 += &poly2;

    assert_eq!(poly1.to_string(), "P[(10)x + (3)]");
    assert_eq!(poly2.to_string(), "P[(-10)x + (1)]");

    poly1 += &poly2;
    assert_eq!(poly1.to_string(), "P[(4)]");
  }
}

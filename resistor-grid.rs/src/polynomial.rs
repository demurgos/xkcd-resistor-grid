use std::fmt;
use std::ops;
use std::default;
use arithmetic::Arithmetic;

#[derive(Clone, PartialEq, Debug)]
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

impl<T: Arithmetic> ops::Neg for Polynomial<T> {
  type Output = Polynomial<T>;

  fn neg(self) -> Polynomial<T> {
    Polynomial::<T> {
      coefficients: self.coefficients
        .iter()
        .map(|x: &T| -> T {-x.clone()})
        .collect()
    }
  }
}

impl<T: Arithmetic> ops::SubAssign<Polynomial<T>> for Polynomial<T> {
  fn sub_assign(&mut self, rhs: Polynomial<T>) {
    self.sub_assign(&rhs)
  }
}

impl<'a, T: Arithmetic> ops::SubAssign<&'a Polynomial<T>> for Polynomial<T> {
  fn sub_assign(&mut self, rhs: &'a Polynomial<T>) {
    match rhs.degree() {
      PolynomialDegree::NegativeInfinity => {},
      PolynomialDegree::FiniteValue(rhs_deg) => {
        match self.degree() {
          PolynomialDegree::NegativeInfinity => {
            self.coefficients = rhs.coefficients
              .iter()
              .map(|x: &T| -> T {-x.clone()})
              .collect();
          },
          PolynomialDegree::FiniteValue(self_deg) => {
            if self_deg > rhs_deg {
              for (degree, coeff) in rhs.coefficients.iter().enumerate() {
                self.coefficients[degree] -= coeff.clone()
              }
            } else {
              let mut tmp_coefficients = rhs.coefficients.clone();
              for (degree, coeff) in self.coefficients.iter().enumerate() {
                tmp_coefficients[degree] -= coeff.clone()
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

impl<T: Arithmetic> ops::Sub<Polynomial<T>> for Polynomial<T> {
  type Output = Polynomial<T>;

  fn sub(self, rhs: Polynomial<T>) -> Polynomial<T> {
    let mut result = self.clone();
    result -= rhs;
    result
  }
}

impl<T: Arithmetic> ops::Mul<Polynomial<T>> for Polynomial<T> {
  type Output = Polynomial<T>;

  fn mul(self, rhs: Polynomial<T>) -> Polynomial<T> {
    &self * &rhs
  }
}

impl<T: Arithmetic> default::Default for Polynomial<T> {
  fn default() -> Polynomial<T> {
    Polynomial::<T> {coefficients: vec![T::default(); 0usize]}
  }
}

impl<T: Arithmetic> ops::DivAssign<Polynomial<T>> for Polynomial<T> {
  fn div_assign(&mut self, rhs: Polynomial<T>) {
    match rhs.degree() {
      PolynomialDegree::NegativeInfinity => {
        panic!("Division by null polynomial");
      },
      PolynomialDegree::FiniteValue(rhs_deg) => {
        match self.degree() {
          PolynomialDegree::NegativeInfinity => {},
          PolynomialDegree::FiniteValue(self_deg) => {
            let mut remainder = self.coefficients.clone();
            let divisor = rhs.coefficients.clone();
            let quotient_deg: usize = rhs_deg - self_deg;
            let mut quotient = vec![T::default(); quotient_deg + 1];

            while remainder.len() >= divisor.len() {
              let mut main_quotient = remainder[remainder.len() - 1].clone();
              main_quotient /= divisor[divisor.len() - 1].clone();
              let deg = remainder.len() - divisor.len();
              quotient[deg] = main_quotient.clone();
              for i in deg..(remainder.len() - 1) {
                remainder[i] -= main_quotient.clone() * divisor[i - deg].clone();
              }
              remainder.pop();
            }
            Polynomial{coefficients: quotient};
          }
        }
      }
    }
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
          if *coeff == T::default() {
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

  #[test]
  fn test_div_assign() {
    let mut p1 = Polynomial::<i32> {coefficients: vec![-1, 0, 1]};
    let p2 = Polynomial::<i32> {coefficients: vec![1, 1]};
    p1 /= p2.clone();

    assert_eq!(p1.to_string(), "P[(-1)x + (1)]");
    assert_eq!(p2.to_string(), "P[(1)x + (1)]");
  }
}

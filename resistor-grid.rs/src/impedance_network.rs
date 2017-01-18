use arithmetic::Arithmetic;
use matrix::Matrix;

use std::ops;
use std::cmp;

#[derive(Clone, Debug, PartialEq)]
pub struct ImpedanceNetwork<T: Arithmetic> {
  size: usize,
  data: Vec<T>
}

impl<T: Arithmetic> ImpedanceNetwork<T> {
  /// Size: number of nodes
  pub fn new(size: usize) -> ImpedanceNetwork<T> {
    let number_of_edges = (size * size - 1) / 2;

    ImpedanceNetwork::<T> {size: size, data: vec![T::default(); number_of_edges]}
  }

  // Exchange the id of two nodes (and ensures that the values on the edges still match)
  fn swap_nodes(&mut self, n1: usize, n2: usize) {
    assert!(n1 < self.size);
    assert!(n2 < self.size);

    if n1 == n2 {
      return
    }
    let mut n1_neighbours: Vec<T> = vec!();
    let mut n2_neighbours: Vec<T> = vec!();
    for i in 0..self.size {
      if i != n1 {
        n1_neighbours.push(self[[n1, i]].clone());
      }
      if i != n2 {
        n2_neighbours.push(self[[n2, i]].clone());
      }
    }

    assert_eq!(n1_neighbours.len(), self.size - 1);
    assert_eq!(n2_neighbours.len(), self.size - 1);

    for neighbour_index in 0..(self.size - 1) {
      let target_index = if neighbour_index < n1 {neighbour_index} else {neighbour_index + 1};
      if target_index != n2 {
        self[[n2, target_index]] = n1_neighbours[neighbour_index].clone();
      }
    }
    for neighbour_index in 0..(self.size - 1) {
      let target_index = if neighbour_index < n2 {neighbour_index} else {neighbour_index + 1};
      if target_index != n1 {
        self[[n1, target_index]] = n2_neighbours[neighbour_index].clone();
      }
    }
  }

  fn get_matrix(&self, additive_identity_element: T, multiplicative_identity_element: T) -> Matrix<T> {
    let size = 1 + (self.size * (self.size - 1)) / 2;
    let mut mat = Matrix::<T>::new(size);
    mat[[0, 0]] = multiplicative_identity_element;
    mat
  }
}

impl<T: Arithmetic> ops::Index<[usize; 2]> for ImpedanceNetwork<T> {
  type Output = T;

  fn index(&self, index: [usize; 2]) -> &T {
    assert!(index[0] != index[1], "Cannot access impedance on the same node");
    let max_index = cmp::max(index[0], index[1]);
    let min_index = cmp::min(index[0], index[1]);
    assert!(max_index < self.size, "Cannot get edge with node index bigger than number of nodes.");

    let mut result: usize = 0;
    for n in 0..min_index {
      result += self.size - 1 - n;
    }
    result += max_index - 1 - min_index;

    &self.data[result]
  }
}

impl<T: Arithmetic> ops::IndexMut<[usize; 2]> for ImpedanceNetwork<T> {
  fn index_mut(&mut self, index: [usize; 2]) -> &mut T {
    assert!(index[0] != index[1], "Cannot access impedance on the same node");
    let max_index = cmp::max(index[0], index[1]);
    let min_index = cmp::min(index[0], index[1]);
    assert!(max_index < self.size, "Cannot get edge with node index bigger than number of nodes.");

    let mut result: usize = 0;
    for n in 0..min_index {
      result += self.size - 1 - n;
    }
    result += max_index - 1 - min_index;

    &mut self.data[result]
  }
}

#[cfg(test)]
mod tests {
  use super::ImpedanceNetwork;

  #[test]
  fn test_new() {
    let net: ImpedanceNetwork<i32> = ImpedanceNetwork::new(4);
    assert_eq!(net.size, 4);
  }

  #[test]
  fn test_swap() {
    let mut net: ImpedanceNetwork<i32> = ImpedanceNetwork::new(4);

    net[[0, 1]] = 1001;
    net[[0, 2]] = 1002;
    net[[0, 3]] = 1003;
    net[[0, 3]] = 1003;
    net[[1, 2]] = 1012;
    net[[1, 3]] = 1013;
    net[[2, 3]] = 1023;

    net.swap_nodes(1, 2);

    assert_eq!(net[[0, 1]], 1002);
    assert_eq!(net[[0, 2]], 1001);
    assert_eq!(net[[0, 3]], 1003);
    assert_eq!(net[[1, 2]], 1012);
    assert_eq!(net[[1, 3]], 1023);
    assert_eq!(net[[2, 3]], 1013);
  }
}

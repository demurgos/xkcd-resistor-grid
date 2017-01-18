extern crate resistor_grid;

fn main() {
  println!("Hello, world!!!!");
  let a: i32 = 20;
  resistor_grid::matrix::foo(a);
  let foo = resistor_grid::impedance_network::ImpedanceNetwork::<i32>::new(10);
  println!("{:?}", foo);
}

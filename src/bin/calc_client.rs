extern crate rustfun;

use rustfun::calc::calc::*;
use rustfun::calc::calc_grpc::*;

fn main() {
  let client = CalculatorClient::new("localhost", 50051, false).unwrap();

  let mut input = Input::new();
  input.set_op1(2);
  input.set_op2(3);
  input.set_op(Input_Op::ADD);

  let resp = client.Calculate(input);
  println!("{:?}", resp);
}

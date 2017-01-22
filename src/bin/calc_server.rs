extern crate rustfun;
extern crate grpc;

use rustfun::calc::calc::*;
use rustfun::calc::calc_grpc::*;

use std::thread;

use grpc::result::GrpcResult;

struct CalculatorImpl;

impl Calculator for CalculatorImpl {
  fn Calculate(&self, input: Input) -> GrpcResult<Output> {
    let mut output = Output::new();
    let out_val = match input.get_op() {
      Input_Op::ADD => input.get_op1() + input.get_op2(),
      Input_Op::SUB => input.get_op1() - input.get_op2(),
      Input_Op::MUL => input.get_op1() * input.get_op2(),
      Input_Op::DIV => input.get_op1() / input.get_op2()
    };
    output.set_out(out_val);
    Ok(output)
  }
}

fn main() {
  // NOTE: cannot replace '_server' with '_' otherwise it will be
  // optimized away and Rust won't execute this line.
  let _server = CalculatorServer::new("[::]:50051", CalculatorImpl);

  loop {
    thread::park();
  }
}

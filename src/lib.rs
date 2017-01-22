#![feature(box_syntax, box_patterns)]
#![feature(rc_counts)]
#![feature(ptr_eq)]

extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

mod deref;
mod macros;
mod bst;
mod dlist;
pub mod calc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

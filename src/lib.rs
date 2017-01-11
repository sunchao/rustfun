#![feature(box_syntax, box_patterns)]
#![feature(rc_counts)]
#![feature(ptr_eq)]

mod deref;
mod macros;
mod bst;
mod dlist;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

use std::fmt::Debug;
use std::rc::Rc;
use std::rc::Weak;

struct Node<T> where T: Debug {
  data: T,
  prev: Option<Weak<T>>,
  next: Option<Rc<T>>
}

impl<T> Node<T> where T: Debug {
  pub fn new(t: T) -> Self { Node { data: t, prev: None, next: None } }
}

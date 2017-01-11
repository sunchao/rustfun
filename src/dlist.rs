use std::fmt::Debug;
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> where T: Debug {
  data: T,
  prev: Link<T>,
  next: Link<T>
}

impl<T> Node<T> where T: Debug {
  pub fn new(t: T) -> Self { Node { data: t, prev: None, next: None } }
}

struct List<T> where T: Debug {
  head: Link<T>,
  tail: Link<T>,
  size: usize
}

impl<T> List<T> where T: Debug {
  pub fn new() -> Self { List { head: None, tail: None, size: 0 } }

  pub fn is_empty(&self) -> bool {
    self.head.is_none()
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn add(&mut self, t: T) {
    let r = Rc::new(RefCell::new(Node::new(t)));
    match self.head {
      None => {
        {
          let mut r_uref = r.borrow_mut();
          r_uref.prev = Some(r.clone());
          r_uref.next = Some(r.clone());
        }
        self.tail = Some(r.clone());
        self.head = Some(r.clone());
      },
      Some(_) => {
        {
          let tail_rc = self.tail.as_mut().unwrap();
          let mut tail_uref = tail_rc.borrow_mut();
          tail_uref.next = Some(r.clone());
        }
        {
          let head_rc = self.head.as_mut().unwrap();
          let mut head_uref = head_rc.borrow_mut();
          head_uref.prev = Some(r.clone());
        }
        {
          let head_rc = self.head.as_mut().unwrap();
          let tail_rc = self.tail.as_mut().unwrap();
          let mut r_uref = r.borrow_mut();
          r_uref.next = Some(head_rc.clone());
          r_uref.prev = Some(tail_rc.clone());
        }
        self.tail = Some(r);
      }
    }
    self.size += 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // TODO: test links
  #[test]
  fn test_add() {
    let mut list = List::new();
    assert_eq!(list.is_empty(), true);
    list.add(1);
    assert_eq!(1, list.size());
    list.add(2);
    assert_eq!(2, list.size());
    list.add(3);
    assert_eq!(3, list.size());
    assert!(!list.is_empty());

    assert!(list.head.is_some());
    let h1 = list.head.as_ref().unwrap();
    let h1_uref = h1.borrow();

    assert_eq!(h1_uref.data, 1);
    assert!(h1_uref.prev.is_some());
    assert!(h1_uref.next.is_some());

    let h2 = h1_uref.next.as_ref().unwrap();
    let h2_uref = h2.borrow();

    let h3 = h1_uref.prev.as_ref().unwrap();
    let h3_uref = h3.borrow();

    assert_eq!(h2_uref.data, 2);
    assert!(h2_uref.prev.is_some());
    assert!(h2_uref.next.is_some());

    assert_eq!(h3_uref.data, 3);
    assert!(h3_uref.prev.is_some());
    assert!(h3_uref.next.is_some());
  }
}

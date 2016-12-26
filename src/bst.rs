use std::fmt::Debug;
use std::cmp::Ordering;
use std::mem;

/// A BST implementation, whose instances own the nodes.
struct Node<T> where T: Debug + Ord {
  left: Option<Box<Node<T>>>,
  right: Option<Box<Node<T>>>,
  data: T
}

impl<T> Node<T> where T: Debug + Ord {
  fn new(d: T) -> Self { Node { left: None, right: None, data: d } }
}

struct BST<T> where T: Debug + Ord {
  root: Option<Box<Node<T>>>
}

impl<T> BST<T> where T: Debug + Ord {
  /// Instantiate a new empty BST
  fn new() -> Self { BST { root: None } }

  fn add(&mut self, data: T) {
    let old = mem::replace(&mut self.root, None);
    mem::replace(&mut self.root, BST::add_helper(old, data));
  }

  fn add_helper(node: Option<Box<Node<T>>>, data: T) -> Option<Box<Node<T>>> {
    match node {
      None => {
        println!("returning #1");
        return Some(box Node::new(data))
      },
      Some(box mut n) => {
        match data.cmp(&n.data) {
          Ordering::Less => {
            let old_left = mem::replace(&mut n.left, None);
            mem::replace(&mut n.left, BST::add_helper(old_left, data));
            Some(box n)
          },
          Ordering::Equal => {
            n.data = data;
            Some(box n)
          },
          Ordering::Greater => {
            let old_right = mem::replace(&mut n.right, None);
            mem::replace(&mut n.right, BST::add_helper(old_right, data));
            Some(box n)
          }
        }
      }
    }
  }

  fn get(&self, data: T) -> bool {
    BST::get_helper(&self.root, data)
  }

  fn get_helper(node: &Option<Box<Node<T>>>, data: T) -> bool {
    match *node {
      None => false,
      Some(box ref n) => {
        match data.cmp(&n.data) {
          Ordering::Less => BST::get_helper(&n.left, data),
          Ordering::Equal => true,
          Ordering::Greater => BST::get_helper(&n.right, data)
        }
      }
    }
  }

  fn to_string(&self) -> String {
    BST::to_string_helper(&self.root, 0)
  }

  fn to_string_helper(node: &Option<Box<Node<T>>>, indent: usize) -> String {
    match *node {
      None => return String::from(""),
      Some(box ref n) => {
        let mut x = String::new();
        for i in 0..indent {
          x += "-"
        }
        x += format!("{:?}", n.data).as_str();
        x += BST::to_string_helper(&n.left, indent + 1).as_str();
        x += BST::to_string_helper(&n.right, indent + 1).as_str();
        return x
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bst() {
    let mut bst : BST<u32> = BST::new();
    bst.add(3);
    bst.add(2);
    println!("{}", bst.to_string().as_str());
    assert_eq!(true, bst.get(3));
  }
}

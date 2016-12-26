use std::fmt::Debug;
use std::cmp::Ordering;
use std::mem;

struct Node<T> where T: Debug + Ord {
  left: Option<Box<Node<T>>>,
  right: Option<Box<Node<T>>>,
  data: T,
  size: usize
}

impl<T> Node<T> where T: Debug + Ord {
  fn new(d: T) -> Self { Node { left: None, right: None, data: d, size: 1 } }
  fn size(node: &Option<Box<Node<T>>>) -> usize {
    match *node {
      None => 0,
      Some(box ref n) => n.size
    }
  }
}

struct BST<T> where T: Debug + Ord {
  root: Option<Box<Node<T>>>
}

impl<T> BST<T> where T: Debug + Ord {
  fn new() -> Self { BST { root: None } }

  fn add(&mut self, data: T) {
    BST::swap_data(&mut self.root, data);
  }

  fn add_helper(node: Option<Box<Node<T>>>, data: T) -> Option<Box<Node<T>>> {
    let mut new_node = match node {
      None => box Node::new(data),
      Some(box mut n) => {
        match data.cmp(&n.data) {
          Ordering::Less => BST::swap_data(&mut n.left, data),
          Ordering::Equal => n.data = data,
          Ordering::Greater => BST::swap_data(&mut n.right, data)
        };
        box n
      }
    };
    new_node.size = 1 + Node::size(&new_node.left) + Node::size(&new_node.right);
    Some(new_node)
  }

  fn swap_data(node: &mut Option<Box<Node<T>>>, data: T) {
    let old_node = mem::replace(node, None);
    mem::replace(node, BST::add_helper(old_node, data));
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
        for _ in 0..indent {
          x += "-"
        }
        x += format!("{:?}", n.data).as_str();
        x += BST::to_string_helper(&n.left, indent + 1).as_str();
        x += BST::to_string_helper(&n.right, indent + 1).as_str();
        return x
      }
    }
  }

  fn size(&self) -> usize {
    Node::size(&self.root)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bst_add() {
    let mut bst : BST<u32> = BST::new();
    bst.add(3);
    bst.add(2);
    bst.add(4);
    assert_eq!(true, bst.get(4));
    assert_eq!(true, bst.get(3));
    assert_eq!(true, bst.get(2));
    assert_eq!(false, bst.get(5));
  }

  #[test]
  fn test_bst_size() {
    let mut bst = BST::new();
    bst.add(2);
    bst.add(1);
    bst.add(4);
    bst.add(3);
    assert_eq!(4, bst.size());
  }
}
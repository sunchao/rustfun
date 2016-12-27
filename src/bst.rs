use std::fmt::Debug;
use std::cmp::Ordering;
use std::mem;

type NodeType<T> = Option<Box<Node<T>>>;

struct Node<T> where T: Debug + Ord {
  left: NodeType<T>,
  right: NodeType<T>,
  data: T,
  size: usize
}

impl<T> Node<T> where T: Debug + Ord {
  fn new(d: T) -> Self { Node { left: None, right: None, data: d, size: 1 } }
  fn size(node: &NodeType<T>) -> usize {
    match *node {
      None => 0,
      Some(box ref n) => n.size
    }
  }
}

struct BST<T> where T: Debug + Ord {
  root: NodeType<T>
}

impl<T> BST<T> where T: Debug + Ord {
  fn new() -> Self { BST { root: None } }

  fn add(&mut self, data: T) {
    BST::swap_data(&mut self.root, |n| BST::add_helper(n, data));
  }

  fn add_helper(node: NodeType<T>, data: T) -> NodeType<T> {
    let mut new_node = match node {
      None => box Node::new(data),
      Some(box mut n) => {
        match data.cmp(&n.data) {
          Ordering::Less => BST::swap_data(&mut n.left, |n| BST::add_helper(n, data)),
          Ordering::Equal => n.data = data,
          Ordering::Greater => BST::swap_data(&mut n.right, |n| BST::add_helper(n, data))
        };
        box n
      }
    };
    new_node.size = 1 + Node::size(&new_node.left) + Node::size(&new_node.right);
    Some(new_node)
  }

  // Applying function `f` on `node`, and replace the value of `node` with
  // the result of the call result from `f`.
  fn swap_data<F>(node: &mut NodeType<T>, f: F)
    where F: FnOnce(NodeType<T>) -> NodeType<T> {
    let old_node = mem::replace(node, None);
    mem::replace(node, f(old_node));
  }

  fn get(&self, data: T) -> bool {
    BST::get_helper(&self.root, data)
  }

  fn get_helper(node: &NodeType<T>, data: T) -> bool {
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

  fn min(&self) -> Option<&T> {
    match self.root {
      None => None,
      Some(box ref n) => Some(BST::min_helper(n))
    }
  }

  fn min_helper(node: &Node<T>) -> &T {
    match node.left {
      None => &node.data,
      Some(box ref l) => BST::min_helper(l)
    }
  }

  fn max(&self) -> Option<&T> {
    match self.root {
      None => None,
      Some(box ref n) => Some(BST::max_helper(n))
    }
  }

  fn max_helper(node: &Node<T>) -> &T {
    match node.right {
      None => &node.data,
      Some(box ref l) => BST::max_helper(l)
    }
  }

  fn delete_min(&mut self) {
    if self.root.is_some() {
      BST::swap_data(&mut self.root, |n| BST::delete_min_helper(n.unwrap()))
    }
  }

  fn delete_min_helper(mut node: Box<Node<T>>) -> NodeType<T> {
    match node.left {
      None => node.right,
      Some(_) => {
        BST::swap_data(&mut node.left, |n| BST::delete_min_helper(n.unwrap()));
        node.size = 1 + Node::size(&node.left) + Node::size(&node.right);
        Some(node)
      }
    }
  }

  fn delete_max(&mut self) {
    if self.root.is_some() {
      BST::swap_data(&mut self.root, |n| BST::delete_max_helper(n.unwrap()));
    }
  }

  fn delete_max_helper(mut node: Box<Node<T>>) -> NodeType<T> {
    match node.right {
      None => node.left,
      Some(_) => {
        BST::swap_data(&mut node.right, |n| BST::delete_max_helper(n.unwrap()));
        node.size = 1 + Node::size(&node.left) + Node::size(&node.right);
        Some(node)
      }
    }
  }

  fn to_string(&self) -> String {
    BST::to_string_helper(&self.root, 0)
  }

  fn to_string_helper(node: &NodeType<T>, indent: usize) -> String {
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

  #[test]
  fn test_bst_min_max() {
    let mut bst = BST::new();
    bst.add(2);
    bst.add(1);
    bst.add(4);
    bst.add(3);
    assert_eq!(1, *bst.min().unwrap());
    assert_eq!(4, *bst.max().unwrap());
  }

  #[test]
  fn test_bst_delete_min() {
    let mut bst = BST::new();
    bst.add(2);
    bst.add(1);
    bst.add(4);
    assert_eq!(1, *bst.min().unwrap());
    assert_eq!(3, bst.size());
    bst.delete_min();
    assert_eq!(false, bst.get(3));
    assert_eq!(2, *bst.min().unwrap());
    assert_eq!(2, bst.size());
  }

  #[test]
  fn test_bst_delete_max() {
    let mut bst = BST::new();
    bst.add(2);
    bst.add(1);
    bst.add(4);
    bst.add(3);
    assert_eq!(4, *bst.max().unwrap());
    assert_eq!(4, bst.size());
    bst.delete_max();
    assert_eq!(false, bst.get(4));
    assert_eq!(3, *bst.max().unwrap());
    assert_eq!(3, bst.size());
  }
}

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

  /// Delete the minimum node from the tree and return it.
  /// If the tree is empty, return `None` instead.
  fn delete_min(&mut self) -> NodeType<T> {
    match self.root {
      None => None,
      Some(_) => {
        let old_root = mem::replace(&mut self.root, None);
        let (min, new_root) = BST::delete_min_helper(old_root.unwrap());
        self.root = new_root;
        min
      }
    }
  }

  fn delete_min_helper(mut node: Box<Node<T>>) -> (NodeType<T>, NodeType<T>) {
    match node.left {
      None => {
        let right = mem::replace(&mut node.right, None);
        (Some(node), right)
      },
      Some(_) => {
        let old_left = mem::replace(&mut node.left, None);
        let (min, new_left) = BST::delete_min_helper(old_left.unwrap());
        node.left = new_left;
        node.size = 1 + Node::size(&node.left) + Node::size(&node.right);
        (min, Some(node))
      }
    }
  }

  /// Delete the maximum node from the tree and return it.
  /// If the tree is empty, return `None` instead.
  fn delete_max(&mut self) -> NodeType<T> {
    match self.root {
      None => None,
      Some(_) => {
        let old_root = mem::replace(&mut self.root, None);
        let (max, new_root) = BST::delete_max_helper(old_root.unwrap());
        self.root = new_root;
        max
      }
    }
  }

  fn delete_max_helper(mut node: Box<Node<T>>) -> (NodeType<T>, NodeType<T>) {
    match node.right {
      None => {
        let left = mem::replace(&mut node.left, None);
        (Some(node), left)
      },
      Some(_) => {
        let old_right = mem::replace(&mut node.right, None);
        let (max, new_right) = BST::delete_max_helper(old_right.unwrap());
        node.right = new_right;
        node.size = 1 + Node::size(&node.left) + Node::size(&node.right);
        (max, Some(node))
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
    bst.add(4);
    bst.add(1);
    bst.add(2);
    assert_eq!(true, bst.get(1));
    assert_eq!(true, bst.get(4));
    assert_eq!(true, bst.get(2));
    assert_eq!(1, *bst.min().unwrap());
    assert_eq!(3, bst.size());
    let ref min = bst.delete_min();
    assert_eq!(true, min.is_some());
    assert_eq!(1, min.as_ref().unwrap().data);
    assert_eq!(false, bst.get(1));
    assert_eq!(true, bst.get(4));
    assert_eq!(true, bst.get(2));
    assert_eq!(2, *bst.min().unwrap());
    assert_eq!(2, bst.size());
  }

  #[test]
  fn test_bst_delete_max() {
    let mut bst = BST::new();
    bst.add(1);
    bst.add(4);
    bst.add(2);
    assert_eq!(true, bst.get(1));
    assert_eq!(true, bst.get(4));
    assert_eq!(true, bst.get(2));
    assert_eq!(4, *bst.max().unwrap());
    assert_eq!(3, bst.size());
    let ref max = bst.delete_max();
    assert_eq!(true, max.is_some());
    assert_eq!(4, max.as_ref().unwrap().data);
    assert_eq!(true, bst.get(1));
    assert_eq!(false, bst.get(4));
    assert_eq!(true, bst.get(2));
    assert_eq!(2, *bst.max().unwrap());
    assert_eq!(2, bst.size());
  }
}

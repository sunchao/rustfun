use std::fmt::Debug;
use std::cmp::Ordering;
use std::mem;

type NodeType<K, V> = Option<Box<Node<K, V>>>;

struct Node<K, V> where K: Debug + Ord, V: Debug {
  left: NodeType<K, V>,
  right: NodeType<K, V>,
  key: K,
  value: V,
  size: usize
}

impl<K, V> Node<K, V> where K: Debug + Ord, V: Debug {
  pub fn new(k: K, v: V) -> Self { Node { left: None, right: None, key: k, value: v, size: 1 } }
  pub fn size(node: &NodeType<K, V>) -> usize {
    match *node {
      None => 0,
      Some(box ref n) => n.size
    }
  }
}

struct BST<K, V> where K: Debug + Ord, V: Debug {
  root: NodeType<K, V>
}

impl<K, V> BST<K, V> where K: Debug + Ord, V: Debug {
  pub fn new() -> Self { BST { root: None } }

  pub fn is_empty(&self) -> bool {
    self.root.is_none()
  }

  pub fn size(&self) -> usize {
    Node::size(&self.root)
  }

  pub fn put(&mut self, key: K, value: V) {
    BST::swap_data(&mut self.root, |n| BST::add_helper(n, key, value));
  }

  fn add_helper(node: NodeType<K, V>, key: K, value: V) -> NodeType<K, V> {
    let mut new_node = match node {
      None => box Node::new(key, value),
      Some(box mut n) => {
        match key.cmp(&n.key) {
          Ordering::Less => BST::swap_data(&mut n.left, |n| BST::add_helper(n, key, value)),
          Ordering::Equal => n.value = value,
          Ordering::Greater => BST::swap_data(&mut n.right, |n| BST::add_helper(n, key, value))
        };
        box n
      }
    };
    new_node.size = 1 + Node::size(&new_node.left) + Node::size(&new_node.right);
    Some(new_node)
  }

  // Applying function `f` on `node`, and replace the value of `node` with
  // the result of the call result from `f`.
  fn swap_data<F>(node: &mut NodeType<K, V>, f: F)
    where F: FnOnce(NodeType<K, V>) -> NodeType<K, V> {
    let old_node = mem::replace(node, None);
    mem::replace(node, f(old_node));
  }

  pub fn get(&self, key: K) -> Option<&V> {
    BST::get_helper(&self.root, key)
  }

  fn get_helper(node: &NodeType<K, V>, key: K) -> Option<&V> {
    match *node {
      None => None,
      Some(box ref n) => {
        match key.cmp(&n.key) {
          Ordering::Less => BST::get_helper(&n.left, key),
          Ordering::Equal => Some(&n.value),
          Ordering::Greater => BST::get_helper(&n.right, key)
        }
      }
    }
  }

  /// Find the minimum key from the BST. Return `None` if the tree is empty.
  pub fn min_key(&self) -> Option<&K> {
    match self.root {
      None => None,
      Some(box ref n) => Some(BST::min_key_helper(n))
    }
  }

  fn min_key_helper(node: &Node<K, V>) -> &K {
    match node.left {
      None => &node.key,
      Some(box ref l) => BST::min_key_helper(l)
    }
  }

  /// Find the maximum key from the BST. Return `None` if the tree is empty.
  pub fn max_key(&self) -> Option<&K> {
    match self.root {
      None => None,
      Some(box ref n) => Some(BST::max_key_helper(n))
    }
  }

  fn max_key_helper(node: &Node<K, V>) -> &K {
    match node.right {
      None => &node.key,
      Some(box ref l) => BST::max_key_helper(l)
    }
  }

  /// Delete the minimum node from the tree and return it.
  /// If the tree is empty, return `None` instead.
  pub fn delete_min(&mut self) -> NodeType<K, V> {
    match self.root {
      None => None,
      Some(_) => {
        let old_root = mem::replace(&mut self.root, None);
        let (min, new_root) = BST::delete_min_helper(old_root);
        self.root = new_root;
        min
      }
    }
  }

  fn delete_min_helper(node: NodeType<K, V>) -> (NodeType<K, V>, NodeType<K, V>) {
    match node {
      None => (None, None),
      Some(mut n) => {
        match n.left {
          None => {
            let right = mem::replace(&mut n.right, None);
            (Some(n), right)
          },
          Some(_) => {
            let old_left = mem::replace(&mut n.left, None);
            let (min, new_left) = BST::delete_min_helper(old_left);
            n.left = new_left;
            n.size = 1 + Node::size(&n.left) + Node::size(&n.right);
            (min, Some(n))
          }
        }
      }
    }
  }

  /// Delete the node with the maximum key from the tree and return it.
  /// If the tree is empty, return `None` instead.
  pub fn delete_max(&mut self) -> NodeType<K, V> {
    match self.root {
      None => None,
      Some(_) => {
        let old_root = mem::replace(&mut self.root, None);
        let (max, new_root) = BST::delete_max_helper(old_root);
        self.root = new_root;
        max
      }
    }
  }

  fn delete_max_helper(node: NodeType<K, V>) -> (NodeType<K, V>, NodeType<K, V>) {
    match node {
      None => (None, None),
      Some(mut n) => {
        match n.right {
          None => {
            let left = mem::replace(&mut n.left, None);
            (Some(n), left)
          },
          Some(_) => {
            let old_right = mem::replace(&mut n.right, None);
            let (max, new_right) = BST::delete_max_helper(old_right);
            n.right = new_right;
            n.size = 1 + Node::size(&n.left) + Node::size(&n.right);
            (max, Some(n))
          }
        }
      }
    }
  }

  /// Find any node in the tree with the key `key`, and remove it from the BST.
  pub fn delete(&mut self, key: K) {
    BST::swap_data(&mut self.root, |n| BST::delete_helper(n, key))
  }

  fn delete_helper(node: NodeType<K, V>, key: K) -> NodeType<K, V> {
    match node {
      None => None,
      Some(box mut n) => {
        let mut x = match key.cmp(&n.key) {
          Ordering::Less => {
            BST::swap_data(&mut n.left, |n| BST::delete_helper(n, key));
            n
          },
          Ordering::Greater => {
            BST::swap_data(&mut n.right, |n| BST::delete_helper(n, key));
            n
          },
          Ordering::Equal => {
            if n.right.is_none() {
              return n.left
            } else if n.left.is_none() {
              return n.right
            } else {
              // Both children are present
              let new_left = mem::replace(&mut n.left, None);
              let (new_node, new_right) = BST::delete_min_helper(n.right);
              let mut m = *new_node.unwrap();
              m.left = new_left;
              m.right = new_right;
              m
            }
          }
        };
        x.size = 1 + Node::size(&x.left) + Node::size(&x.right);
        Some(box x)
      }
    }
  }

  pub fn to_string(&self) -> String {
    BST::to_string_helper(&self.root, 0)
  }

  fn to_string_helper(node: &NodeType<K, V>, indent: usize) -> String {
    match *node {
      None => return String::from(""),
      Some(box ref n) => {
        let mut x = String::new();
        for _ in 0..indent {
          x += "-"
        }
        x += format!("{:?}: {:?}", n.key, n.value).as_str();
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
  fn test_bst_add() {
    let mut bst = BST::new();
    bst.put(3, "c");
    bst.put(2, "b");
    bst.put(4, "d");
    assert_eq!(Some(&"d"), bst.get(4));
    assert_eq!(Some(&"c"), bst.get(3));
    assert_eq!(Some(&"b"), bst.get(2));
    assert_eq!(None, bst.get(5));
  }

  #[test]
  fn test_bst_size() {
    let mut bst = BST::new();
    bst.put(2, "b");
    bst.put(1, "a");
    bst.put(4, "d");
    bst.put(3, "c");
    assert_eq!(4, bst.size());
  }

  #[test]
  fn test_bst_min_max() {
    let mut bst = BST::new();
    bst.put(2, "b");
    bst.put(1, "a");
    bst.put(4, "d");
    bst.put(3, "c");
    assert_eq!(Some(&1), bst.min_key());
    assert_eq!(Some(&4), bst.max_key());
  }

  #[test]
  fn test_bst_delete_min() {
    let mut bst = BST::new();
    bst.put(4, "d");
    bst.put(1, "a");
    bst.put(2, "b");
    assert_eq!(Some(&"a"), bst.get(1));
    assert_eq!(Some(&"d"), bst.get(4));
    assert_eq!(Some(&"b"), bst.get(2));
    assert_eq!(Some(&1), bst.min_key());
    assert_eq!(3, bst.size());

    let ref min = bst.delete_min();
    assert_eq!(true, min.is_some());
    assert_eq!(1, min.as_ref().unwrap().key);
    assert_eq!(None, bst.get(1));
    assert_eq!(Some(&"d"), bst.get(4));
    assert_eq!(Some(&"b"), bst.get(2));
    assert_eq!(Some(&2), bst.min_key());
    assert_eq!(2, bst.size());
  }

  #[test]
  fn test_bst_delete_max() {
    let mut bst = BST::new();
    bst.put(1, "a");
    bst.put(4, "d");
    bst.put(2, "b");
    assert_eq!(Some(&"a"), bst.get(1));
    assert_eq!(Some(&"d"), bst.get(4));
    assert_eq!(Some(&"b"), bst.get(2));
    assert_eq!(Some(&4), bst.max_key());
    assert_eq!(3, bst.size());

    let ref max = bst.delete_max();
    assert_eq!(true, max.is_some());
    assert_eq!(4, max.as_ref().unwrap().key);
    assert_eq!(Some(&"a"), bst.get(1));
    assert_eq!(None, bst.get(4));
    assert_eq!(Some(&"b"), bst.get(2));
    assert_eq!(Some(&2), bst.max_key());
    assert_eq!(2, bst.size());
  }

  #[test]
  fn test_bst_delete() {
    let mut bst = BST::new();
    bst.put(4, "d");
    bst.put(1, "a");
    bst.put(2, "b");
    assert_eq!(Some(&"a"), bst.get(1));
    assert_eq!(Some(&"d"), bst.get(4));
    assert_eq!(Some(&"b"), bst.get(2));
    assert_eq!(3, bst.size());

    bst.delete(1);
    assert_eq!(None, bst.get(1));
    assert_eq!(Some(&"d"), bst.get(4));
    assert_eq!(Some(&"b"), bst.get(2));
    assert_eq!(2, bst.size());

    bst.delete(4);
    assert_eq!(None, bst.get(1));
    assert_eq!(None, bst.get(4));
    assert_eq!(Some(&"b"), bst.get(2));
    assert_eq!(1, bst.size());

    bst.delete(2);
    assert_eq!(None, bst.get(1));
    assert_eq!(None, bst.get(4));
    assert_eq!(None, bst.get(2));
    assert_eq!(true, bst.is_empty());
  }
}

use std::ops::Deref;

struct DerefExample<T> {
  value: T
}

impl<T> Deref for DerefExample<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.value
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_deref() {
    let x = DerefExample { value: "foo" };
    assert_eq!("foo", *x);
  }
}

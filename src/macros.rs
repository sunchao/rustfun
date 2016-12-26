macro_rules! foo {
  (x => $e: expr) => (format!("mode X: {}", $e));
  (y => $e: expr) => (format!("mode Y: {}", $e));
}

macro_rules! o_O {
  ( $( $x: expr; [ $( $y: expr ),* ] );* ) =>
    { &[ $($( $x + $y ),*),*] }
}

macro_rules! gen_plus_one {
  ($f: ident) => (
    fn $f(x: i32) -> i32 {
      x + 1
    }
  )
}
gen_plus_one!(plus_one);

#[cfg(test)]
mod tests {
  use super::plus_one;

  #[test]
  fn test_foo() {
    assert_eq!("mode X: 3", foo!(x => 3));
  }

  #[test]
  fn test_o_o() {
    let a: &[i32] = o_O!(10; [1, 2, 3]; 20; [1, 2, 3]);
    assert_eq!([11,12,13,21,22,23], a);
  }

  #[test]
  fn test_gen_plus_one() {
    assert_eq!(2, plus_one(1));
  }
}

use arcsec::input::{Input};

#[test]
fn input_should_only_be_modified_by_pop() {
  let mut input = Input::from_str("foo");
  let first = input.pop();
  let second = input.pop();
  let third = input.pop();
  let fourth = input.pop();
  println!("{} {} {} {}", first, second, third, fourth);
  assert!(first == Some('f'));
  assert!(second == Some('o'));
  assert!(third == Some('o'));
  assert!(fourth == None)
}

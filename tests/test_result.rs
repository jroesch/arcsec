use arcsec::result::*;
use arcsec::input::*;

#[test]
fn map_should_only_modify_result() {
  let increment = |i: int| -> int { i + 1 };
  let input = Input::from_str("unchanged");
  let initial = Success(1, input.clone());
  assert_eq!(initial.map(increment), Success(2, input));
}

use arcsec::parser::*;
use arcsec::result::*;
use arcsec::input::*;

#[test]
fn string_parser_should_match_only_the_literal_string() {
  /* let string_parser = string(str("A string"));
  let here_parser = string(str(" here"));
  let input = Input::from_string(str("A string here"));
  let result = match string_parser.run(input) {
    Failed(err, rest) => fail!(format!("Parse was not successful: {}", err)),
    Success(v, input) => here_parser.run(input)
  };
  assert_eq!(result, Success(" here", "")) */
}

#[test]
fn string_parser_test2() {
  let mut parserA = satisfy(|c: char| c == 'A');
  let input = Input::from_string(String::from_str("A"));
  let result = parserA.run(input);
  let unpacked = match result {
    Failed(e, _) => fail!(e),
    Success(v, _) => v
  };
  println!("{}", unpacked)
}

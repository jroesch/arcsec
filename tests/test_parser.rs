use arcsec::parser::*;
use arcsec::result::*;
use arcsec::ibuffer::*;

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
  let input = IBuffer::new(String::from_str("A"));
  let result = parserA.run(input);
  let unpacked = match result {
    Failed(e, _) => fail!(e),
    Success(v, _) => v
  };
  println!("{}", unpacked)
}

#[test]
fn choice_parser_should_return_the_correct_result_or_failure() {
  let parserA = satisfy(|c: char| c == 'A');
  let parserB = satisfy(|c: char| c == 'B');
  let parserFail = fail_parser(String::from_str("failed!"));
  let input = IBuffer::new(String::from_str("AB"));

  match parserA.or(&parserB).run(input.clone()) {
    Failed(err, _) => fail!(err),
    Success(v, _) => assert_eq!(v, 'A')
  }

  match parserA.or(&parserFail).run(input.clone()) {
    Failed(err, _) => fail!(err),
    Success(v, _) => assert_eq!(v, 'A')
  }

  match parserFail.or(&parserA).run(input.clone()) {
    Failed(err, _) => fail!(err),
    Success(v, _) => assert_eq!(v, 'A')
  }

  match parserFail.or(&parserFail).run(input) {
    Failed(err, _) => assert_eq!(err.as_slice(), "failed!"),
    _ => fail!("this should of failed and not succeded")
  }
}

#[test]
fn skip_while_parser() {

}

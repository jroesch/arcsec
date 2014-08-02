#![feature(struct_variant)]
use std::iter::FromIterator;

#[deriving(Show)]
pub enum Result<T> {
  Failed(String, Input),  //ineff fix this
  Success(T, Input)
}

impl<A> Result<A> {
  fn map<B>(self, f: |A| -> B) -> Result<B> {
    match self {
      Failed(e, remaining) => Failed(e.clone(), remaining),
      Success(v, remaining) => { Success(f(v), remaining) }}
  }
}

#[deriving(Show)]
pub struct Input {
  buffer: Vec<char>
}

impl Input {
  fn from_str(source: &str) -> Input {
    Input { buffer: FromIterator::from_iter(source.chars()) }
  }

  fn from_string(source: String) -> Input {
    Input { buffer: FromIterator::from_iter(source.as_slice().chars()) }
  }

  fn peek(&self) -> Option<char> {
    if self.buffer.len() > 0 {
      Some(self.buffer[0])
    } else {
      None
    }
  }

  fn pop(&mut self) -> Option<char> {
    self.buffer.pop()
  }
  // Eventually allow us to move through a mutable stream in
  // given a "view" of our data allowing for back tracking
  // in the parser
  // fn fast_forward
  // fn rewind
}

#[test]
fn input_should_only_be_modified_by_pop() {
  let input = Input::from_str("foo");
  let first = input.pop();
  let second = input.pop();
  let third = input.pop();
  let four = input.pop();
  assert_equal!(first, Some('f'));
  assert_equal!(second, Some('o'));
  assert_equal!(third, Some('o'));
  assert_equal!(four, None)
}

#[test]
fn map_should_only_modify_result() {
  let increment = |i: int| -> int { i + 1 };
  let initial = Success(1, str("unchanged"));
  assert_eq!(initial.map(increment), Success(2, str("unchanged")));
}

pub trait Parser<A> {
  fn run(&mut self, input : Input) -> Result<A>;
}

pub struct SatisfyParser<'a> {
  predicate: |char|: 'a -> bool
}

impl<'a> SatisfyParser<'a> {
  fn check_sat(&mut self, mut input: Input) -> Result<char> {
    match input.peek() {
      Some(c) =>
        if (self.predicate)(c) {
          match input.pop() {
            None => Failed(str("Failed!"), input),
            Some(c) => Success(c, input)
          }
        }
        else { Failed(str("Failed!"), input) },
      None => Failed(str("fucked"), input)
    }
  }
}

impl<'a> Parser<char> for SatisfyParser<'a> {
  fn run(&mut self, input: Input) -> Result<char> {
    self.check_sat(input)
  }
}

fn satisfy<'a>(pred: |char|: 'a -> bool) -> SatisfyParser<'a> {
    SatisfyParser { predicate: pred }
}

struct StringParser {
    str_match: String
}

fn string<'a>(str: String) -> StringParser {
  StringParser
}

#[test]
fn string_parser_should_match_only_the_literal_string() {
  let string_parser = string(str("A string"));
  let here_parser = string(str("here"));
  let input = Input::from_string("A string here");
  let result = match string_parser.run(input) {
    Failed(err, rest) => fail!(fmt("Parse was not successful: {}", err)),
    Success(v, input) => here_parser.run(input)
  };
  assert_equal!(result, Success(v, ""))
}

fn str(s: &str) -> String { String::from_str(s) }

fn main() {
  let mut parserA = satisfy(|c: char| c == 'A');
  let input = Input::from_string(String::from_str("A"));
  let result = parserA.run(input);
  let unpacked = match result {
    Failed(e, _) => fail!(e),
    Success(v, _) => v
  };
  println!("{}", unpacked)
}

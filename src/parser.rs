use result::*;
use std::rc::Rc;
use ibuffer::IBuffer;
use boxed_closure::*;

pub trait Parser<A> : Clone {
  fn run(&mut self, input : IBuffer) -> Result<A>;
  fn or<P : Parser<A>>(&self, other: &P) -> ChoiceParser<A, Self, P> {
    ChoiceParser { first: (*self).clone(), second: (*other).clone() }
  }
}

fn demandInput(input: IBuffer, start: uint, end: uint) -> Result<String> {
  match input.substring(start, end) {
    None => Failed(String::from_str("Demanded {} input from buffer"), input),
    Some(s) => Success(s, input.advance(end - start))
  }
}

#[deriving(Clone)]
pub struct FailParser<T> {
  msg: String
}

impl<T: Clone> Parser<T> for FailParser<T> {
  fn run(&mut self, input: IBuffer) -> Result<T> {
    Failed(self.msg.clone(), input)
  }
}

#[deriving(Clone)]
// A parser that represents that implements the behavior of satisfy.
pub struct SatisfyParser {
  predicate: BoxedClosure<char, bool>
}

impl SatisfyParser {
  fn check_sat(&self, mut input: IBuffer) -> Result<char> {
    match demandInput(input, 0, 1) {
      Failed(err, rem) => Failed(err, rem),
      Success(str, rem) => {
        let ref pred = self.predicate;
        let char = str.as_slice().chars().next().unwrap();
        if pred.call(char) {
          Success(char, rem)
        } else {
          // Here we backtrack if the parser fails
          Failed(String::from_str("Does not match predicate ..."), rem)
        }
      }
    }
  }
}

impl Parser<char> for SatisfyParser {
  fn run(&mut self, input: IBuffer) -> Result<char> {
    self.check_sat(input)
  }
}

// #[deriving(Clone)]
// struct ThenParser<A, B, P1, P2> {
//   first: P1,
//   second: P2
// }
//
// impl<A, B, P1 : Parser<A>, P2: Parser<B>> Parser<B> for ThenParser<A, B, P1, P2> {
//   fn run(&mut self, input: IBuffer) -> Result<B> {
//     match self.first.run(input) {
//       Failed(err, rest) => Failed(err, rest),
//       Success(_, rest) => self.second.run(rest)
//     }
//   }
// }

//#[deriving(Clone)]
pub struct StringParser {
    match_str: String
}

impl Clone for StringParser {
  fn clone(&self) -> StringParser {
    StringParser { match_str: self.match_str.clone() }
  }
}

impl Parser<String> for StringParser {
  fn run(&mut self, mut input: IBuffer) -> Result<String> {
    fail!("failed")
  }
}

//#[deriving(Clone)]
pub struct ChoiceParser<A, P1, P2> {
  first: P1,
  second: P2
}

impl<A, P1: Parser<A>, P2: Parser<A>> Clone for ChoiceParser<A, P1, P2> {
  fn clone(&self) -> ChoiceParser<A, P1, P2> {
    ChoiceParser { first: self.first.clone(), second: self.second.clone() }
  }
}

impl<A, P1 : Parser<A>, P2: Parser<A>> Parser<A> for ChoiceParser<A, P1, P2> {
  fn run(&mut self, input: IBuffer) -> Result<A> {
    match self.first.run(input) {
      Failed(err, remaining) =>
        self.second.run(remaining),
      success =>
        success
    }
  }
}
pub fn str(s: &str) -> String { String::from_str(s) }

pub fn fail_parser<T>(s: String) -> FailParser<T> {
  FailParser { msg: s }
}

pub fn satisfy(pred: |char| -> bool) -> SatisfyParser {
  SatisfyParser { predicate: BoxedClosure::new(pred) }
}

pub fn string(string: String) -> StringParser {
  StringParser { match_str: string }
}

pub fn is_horizontal_space(c: char) -> bool {
  c == ' ' && c == '\t'
}

pub fn execute_parser<A, P : Parser<A>>(mut parser: P, input: IBuffer) -> A {
  match parser.run(input) {
    Failed(err, remaining) =>
      fail!("Parsing failed with: {}\n\nRemaining Input: {}", err, remaining.to_string()),
    Success(v, remaining) =>
      v
  }
}

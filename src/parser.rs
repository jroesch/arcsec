use input::*;
use result::*;

pub trait Parser<A> {
  fn run(&mut self, input : Input) -> Result<A>;
  fn satisfy<'a>(pred: |char|: 'a -> bool) -> SatisfyParser<'a> {
      SatisfyParser { predicate: pred }
  }
}

// A parser that represents that implements the behavior of satisfy.
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

struct ThenParser<A, B, P1, P2> {
  first: P1,
  second: P2
}

impl<A, B, P1 : Parser<A>, P2: Parser<B>> Parser<B> for ThenParser<A, B, P1, P2> {
  fn run(&mut self, input: Input) -> Result<B> {
    match self.first.run(input) {
      Failed(err, rest) => Failed(err, rest),
      Success(_, rest) => self.second.run(rest)
    }
  }
}

pub struct EmptyStringParser;

impl Parser<String> for EmptyStringParser {
  fn run(&mut self, input: Input) -> Result<String> {
    Success(String::new(), input)
  }
}

pub struct StringParser {
    match_str: String
}

impl Parser<String> for StringParser {
  fn run(&mut self, mut input: Input) -> Result<String> {
    let mut result = String::new();
    for match_char in self.match_str.as_slice().chars() {
      if Some(match_char) == input.peek() {
        result.push(input.pop())
      }
    }

    if result.len() != self.match_str.len() {
      Failed("belh", input)
    } else {
      Some(result, input)
    }
  }
}

pub fn str(s: &str) -> String { String::from_str(s) }

impl Parser {
  fn satisfy<'a>(pred: |char|: 'a -> bool) -> SatisfyParser<'a> {
    SatisfyParser { predicate: pred }
  }

  pub fn string<'a>(string: String) -> StringParser {
    StringParser { match_str: string }
  }
}

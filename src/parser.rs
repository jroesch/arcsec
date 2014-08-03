use input::*;
use result::*;

pub trait Parser<A> {
  fn run(&mut self, input : Input) -> Result<A>;
  fn satisfy<'a>(pred: |char|: 'a -> bool) -> SatisfyParser<'a> {
      SatisfyParser { predicate: pred }
  }
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
    fail!("Not yet") //fself.check_sat(input)
  }
}

struct ThenParser<A, B, P1, P2> {
  first: P1,
  second: P2
}

impl<A, B, P1 : Parser<A>, P2: Parser<B>> Parser<B> for ThenParser<A, B, P1, P2> {
  fn run(&mut self, input: Input) -> Result<B> {
    fail!("okay")
  }
}

fn satisfy<'a>(pred: |char|: 'a -> bool) -> SatisfyParser<'a> {
    SatisfyParser { predicate: pred }
}

pub struct EmptyStringParser;

impl Parser<String> for EmptyStringParser {
  fn run(&mut self, input: Input) -> Result<String> {
    Success(String::new(), input)
  }
}

pub struct StringParser {
    str_match: String
}

pub fn string<'a>(string: String) -> StringParser {
  StringParser { str_match: string }
}

/* impl Parser<String> for StringParser {
  fn run(&mut self, input: Input) -> Result<String> {
    self.str_match.as_slice().chars().map(|c| {
      satisfy(|input_char| input_char == c);
    }).fold(|accum, parser|{
      match accum.run(input) {
        Failed(err, remaining) =>
        Success(v, remaining)
      }
    });
  }
} */

pub fn str(s: &str) -> String { String::from_str(s) }

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

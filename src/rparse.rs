#![feature(struct_variant)]

#[deriving(PartialEq, Show)]
enum Result<T> {
  Failed(String, String),  //ineff fix this
  Success(T, String)
}

impl<A> Result<A> {
  fn map<B>(self, f: |A| -> B) -> Result<B> {
    match self {
      Failed(e, remaining) => Failed(e.clone(), remaining),
      Success(v, remaining) => { Success(f(v), remaining) }}
  }
}

#[test]
fn map_should_only_modify_result() {
  let increment = |i: int| -> int { i + 1 };
  let initial = Success(1, str("unchanged"));
  assert_eq!(initial.map(increment), Success(2, str("unchanged")));
}

struct Parser<'a, T> {
  parser: |String|: 'a -> Result<T>
}

enum Either<E, R> {
  Left(E),
  Right(R)
}

struct Pred<'a, Arg, Res> {
  func: |Arg|: 'a -> Res
}

impl<'a, Arg, Res> Pred<'a, Arg, Res> {
  fn call(&mut self, arg: Arg) -> Res {
    (self.func)(arg)
  }
}

fn alloc_pred<Arg, Res>(f: |Arg| -> Res) -> Pred<Arg, Res> {
  Pred { func: f }
}

impl<'a, T> Parser<'a, T> {
  fn run(self, filename: String, mut input: String) -> Either<String, T> {
    println!("{}", filename);
    match self {
      Parser { parser } => {
        match parser(input) {
          Failed(errmsg, _) => Left(errmsg),
          Success(v, _) => Right(v)
        }
      }
    }
  }
}

pub trait Parser<A> {
  fn run(input : String) -> Result<A> {}
}

struct SatifsyParser<'a> {
  predicate: |char| -> bool
}

impl<'a> SatisfyParser<'a> {
  fn check_sat(&mut self, input: String) -> Result<char> {
    match input.pop_char() {
      Some(c) =>
        if (self.predicate)(c) { Success('a', input) }
        else { Success('a', input) },
      None => Failed(str("fucked"), input)
    }
  }
}

// impl<'a> Parser<char> {
  pub fn satisfy<'a>(pred: &Pred<'a, char, bool>) -> Parser<'a, char> {
    Parser { parser: |mut input: String| {
      match input.pop_char() {
        Some(c) => {
          let pred2 = pred;
          if true { Success('a', input) } else { Success('a', input) }},
        None => Failed(str("fucked"), input)
      }
    }}
  }
    //let my_pred = |c: char| true;
    /*Parser {
      parser: |input: String| {
                match input.pop_char() {
                  Some(first) => match my_pred(first) {
                    false => {
                      input.push_char(first);
                      Failed(str("statisfy: failed when demanding input"), input)
                    },
                    true => Success(first, input)
                  },
                  None => Failed(str("statisfy failed"), input)
                }
              }
    } */
// }

// impl Parser {
// fn satisfy(pred: |A|
//

fn str(s: &str) -> String { String::from_str(s) }

fn main() {
  let parserA = satisfy(&alloc_pred(|c: char| c == 'A'));
  let result = parserA.run(String::from_str("Testing!"), str("ABBBC"));
  let unpacked = match result {
    Left(e) => fail!(e),
    Right(v) => v
  };
  println!("{}", unpacked)
}

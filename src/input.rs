#[deriving(Show, PartialEq, Clone)]
pub struct Input {
  buffer: Vec<char>
}

impl Input {
  pub fn from_str(source: &str) -> Input {
    Input { buffer: FromIterator::from_iter(source.chars()) }
  }

  pub fn from_string(source: String) -> Input {
    Input { buffer: FromIterator::from_iter(source.as_slice().chars()) }
  }

  pub fn peek(&self) -> Option<char> {
    if self.buffer.len() > 0 {
      Some(self.buffer[0])
    } else {
      None
    }
  }

  pub fn pop(&mut self) -> Option<char> {
    self.buffer.shift()
  }

  /* fn consume_if_match(&mut self, str_match: &str) -> Option<(String, Input)> {
    let mut result = String::new();
    for (x, y) in self.buffer.iter().zip(str_match.chars()) {
    }
    let new_input = self.buffer.as_slice().slice_from(result.len());
    fail!("foooo") //Some((result, new_input))
  } */
}

/* fn shrink(string: String): Option<String> {
  String.from_slice(string.slice()
} */


  // Eventually allow us to move through a mutable stream in
  // given a "view" of our data allowing for back tracking
  // in the parser
  // fn fast_forward
  // fn rewind

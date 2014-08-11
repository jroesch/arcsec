/// A IBuffer representation of the current input
use std::rc::Rc;
use std::fmt::{Show, Formatter, FormatError};
use std::mem::transmute;
use raw = std::string::raw;

/// An input buffer that provides different immutable views of the same
/// underlying mutable buffer. We store the underlying buffer as a
/// `Rc<Vec<u8>>`. This allows us to automatically free the buffer when all
/// references go out of scope, and allows us to piggy back on the allocation
/// smarts of `Vec` for the time being.
#[deriving(Clone, PartialEq)]
pub struct IBuffer {
  contents: Rc<Vec<u8>>,
  offset: uint
}

impl IBuffer {
  /// Constructs a new buffer filled with the contents of `s`.
  pub fn new(s: String) -> IBuffer {
    IBuffer { contents: Rc::new(s.into_bytes()), offset: 0 }
  }

  fn subbytes(&self, start: uint, end: uint) -> Option<Vec<u8>> {
    fail!("subbytes!")
  }

  pub fn substring(&self, start: uint, end: uint) -> Option<String> {
    let ref contents = *self.contents;
    let length = contents.len();
    if self.offset + (end - start) <= length {
      unsafe {
        let ptr: *mut u8 = contents.as_ptr() as *mut u8;
        let offset = self.offset as int;
        let slice_start = transmute(ptr.offset(offset));
        Some(raw::from_buf_len(slice_start, end - start))
      }
    } else {
      None
    }
  }

  /// Creates a new IBuffer that views the underlying buffer from a
  /// different offset.
  pub fn advance(&self, n: uint) -> IBuffer {
    IBuffer { contents: self.contents.clone(), offset: self.offset + n }
  }

  /// For debugging and display purposes.
  fn as_string(&self) -> String {
    match String::from_utf8(Vec::from_slice(self.contents.as_slice())) {
      Err(e) => fail!(e),
      Ok(res) => res
    }
  }
}

impl Show for IBuffer {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
    fmt.write(self.contents.as_slice())
  }
}

#[test]
fn advance_followed_by_substring_provides_a_different_view_of_the_buffer() {
  let buffer = IBuffer::new(String::from_str("Hello World!"));
  let s1 = buffer.substring(0, 5);
  let new_buffer = buffer.advance(6);
  let s2 = new_buffer.substring(0, 5);
  match (s1, s2) {
    (Some(str1), Some(str2)) => {
      assert_eq!("Hello", str1.as_slice());
      assert_eq!("World", str2.as_slice())
    },
    _ => fail!("undeflowed the buffer"),
  }
}

#[test]
fn mutating_a_substring_does_not_corrupt_the_underlying_buffer() {
  let buffer = IBuffer::new(String::from_str("Hello World!"));
  let s = buffer.substring(0, 5);
  match s {
    None => fail!("undeflowed the buffer"),
    Some(mut str) => {
      str.push_str(" Jared!");
      assert_eq!(buffer.to_string(), String::from_str("Hello World!"))
      assert_eq!(str, String::from_str("Hello Jared!"))
    }
  }
}

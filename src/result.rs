use input::*;

use std::iter::FromIterator;

#[deriving(Show, PartialEq)]
pub enum Result<T> {
  Failed(String, Input),  //ineff fix this
  Success(T, Input)
}

impl<A> Result<A> {
  pub fn map<B>(self, f: |A| -> B) -> Result<B> {
    match self {
      Failed(e, remaining) => Failed(e.clone(), remaining),
      Success(v, remaining) => { Success(f(v), remaining) }}
  }
}

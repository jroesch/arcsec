use ibuffer::*;
use std::iter::FromIterator;

#[deriving(Show, PartialEq)]
pub enum Result<T> {
  Failed(String, IBuffer),  //ineff fix this
  Success(T, IBuffer)
}

impl<A> Result<A> {
  pub fn map<B>(self, f: |A| -> B) -> Result<B> {
    match self {
      Failed(e, remaining) => Failed(e.clone(), remaining),
      Success(v, remaining) => { Success(f(v), remaining) }}
  }

/*  pub fn flat_map<B>(&self, f: |A| -> Result<B>) -> Result<B> {
    match *self {
      Success(v, remaining) => Success(f(v), remaining),
      Failed(e, remaining) => Failed(e, remaining)
    }
  } */
}

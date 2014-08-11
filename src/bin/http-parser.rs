#![feature(globs)]
// placholder for http parser
//
extern crate arcsec;
use arcsec::parser::*;

fn main() {
  println!("{}", 1u)
}

fn skip_spaces() {
  let single_space = satisfy(is_horizontal_space);
}

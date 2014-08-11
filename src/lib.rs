#![crate_name="arcsec"]
#![feature(struct_variant)]
#![feature(overloaded_calls, unboxed_closures)]
#![feature(globs)]
#![allow(dead_code)]
//#![deny(warnings)]
extern crate core;

pub mod input;
pub mod ibuffer;
pub mod parser;
pub mod result;
pub mod boxed_closure;

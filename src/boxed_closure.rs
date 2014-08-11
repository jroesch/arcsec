use core::raw::{Closure};
use std::rc::Rc;
use std::mem;

/// Total hack to get around closure copying
pub struct BoxedClosure<Arg, Res> {
    closure: Rc<Closure>
}

impl<Arg, Res> BoxedClosure<Arg, Res> {
  pub fn new(f: |Arg| -> Res) -> BoxedClosure<Arg, Res> {
    let boxed_closure: Closure = unsafe { mem::transmute(f) };
    BoxedClosure { closure: Rc::new(boxed_closure) }
  }

  pub fn call(&self, arg: Arg) -> Res {
    let clo = *self.closure;
    let fun: |Arg| -> Res = unsafe { mem::transmute(clo) };
    fun(arg)
  }
}

impl<Arg, Res> Clone for BoxedClosure<Arg, Res> {
  fn clone(&self) -> BoxedClosure<Arg, Res> {
    BoxedClosure { closure: self.closure.clone() }
  }
}

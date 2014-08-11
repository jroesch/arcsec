higher kinded types in rust

struct List<A> { ... }

/* note we could fully apply this or use
trait Functor for List { ... }
trait Functor for List<_> { ... }
  fn (&self<A>

Functor<A> for List

when we implement a normal trait we have a hidden type parameter

if we look at Haskell for example, a simple show class looks like this

class Show a
  show :: a -> String

instance Show String
  show x = x

trait Show {
  fn show(&self) -> String
}

// Here &self is &self is &String
impl Show for String {
  fn show(&self) -> String {
    self.clone()
  }
}

If we move to an example like:

// Could be named anything, just keeping with Haskell
// convention, this represents a container that can be mapped ove
//
class Functor f
  map :: (a -> b) -> f a -> f b

instance Functor [] where
  map f [] = []
  map f (x:xs) = f x : map f xs

trait Functor where Self<_> { ... }

impl Functor for List

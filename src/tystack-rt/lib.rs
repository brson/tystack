#![allow(non_camel_case_types)]

extern crate tystack_core as tscore;

use std::fmt::Debug;
use tscore::{Stack, End};

#[macro_export]
macro_rules! def_fn_1to1 {
    ($self_:ident $name:ident [ $inty0:ty | $outty0:ty ] $ex:expr) => {
        #[allow(non_camel_case_types)]
        pub trait $name<R2, R1> {
            fn $name($self_) -> Stack<Stack<R2, R1>, $outty0>;
        }
        impl <R2, R1> $name<R2, R1> for Stack<Stack<R2, R1>, $inty0> {
            fn $name($self_) -> Stack<Stack<R2, R1>, $outty0> {
                $ex
            }
        }
    }
}

#[macro_export]
macro_rules! def_fn_2to1 {
    ($self_:ident $name:ident [ $inty1:ty, $inty0:ty | $outty0:ty ] $ex:expr) => {
        #[allow(non_camel_case_types)]
        pub trait $name<R2, R1> {
            fn $name($self_) -> Stack<Stack<R2, R1>, $outty0>;
        }
        impl <R2, R1> $name<R2, R1> for Stack<Stack<Stack<R2, R1>, $inty1>, $inty0> {
            fn $name($self_) -> Stack<Stack<R2, R1>, $outty0> {
                $ex
            }
        }
    }
}

pub trait new {
    fn new() -> Stack<End, End>;
}
impl new for Stack<End, End> {
    fn new() -> Stack<End, End> {
        Stack(End, End)
    }
}

pub trait push<R1, R2> {
    fn push<V>(self, v: V) -> Stack<Stack<R2, R1>, V>;
}
impl<R1, R2> push<R1, R2> for Stack<R2, R1> {
    fn push<V>(self, v: V) -> Stack<Stack<R2, R1>, V> {
        Stack(Stack(self.0, self.1), v)
    }
}

pub trait rt1<A, R1, R2> {
    fn pop(self) -> (Stack<R2, R1>, A);
    fn apply<F, R>(self, f: F) -> R
        where F: FnOnce(Stack<R2, R1>, A) -> R;
    fn recurse<F>(self, f: F) -> Stack<Stack<R2, R1>, A>
        where F: FnOnce(Stack<Stack<End, End>, A>) -> Stack<Stack<End, End>, A>;
    fn debug(self) -> Stack<Stack<R2, R1>, A>
        where A: Debug;
}

impl<A, R1, R2> rt1<A, R1, R2> for Stack<Stack<R2, R1>, A> {
    fn pop(self) -> (Stack<R2, R1>, A) {
        (self.0, self.1)
    }

    fn apply<F, R>(self, f: F) -> R
        where F: FnOnce(Stack<R2, R1>, A) -> R
    {
        let (stack, a) = self.pop();
        f(stack, a)
    }

    fn recurse<F>(self, f: F) -> Stack<Stack<R2, R1>, A>
        where F: FnOnce(Stack<Stack<End, End>, A>) -> Stack<Stack<End, End>, A>
    {
        let (stack, a) = self.pop();
        let newstack = Stack::new().push(a);
        let newstack = f(newstack);
        let (_, b) = newstack.pop();
        let stack = stack.push(b);
        stack
    }

    fn debug(self) -> Stack<Stack<R2, R1>, A>
        where A: Debug
    {
        let (stack, a) = self.pop();
        println!("{:?}", a);
        stack.push(a)
    }
}

pub trait if_<R1, R2> {
    fn if_<F, G, R>(self, f: F, g: G) -> R
        where F: FnOnce(Stack<R2, R1>) -> R,
              G: FnOnce(Stack<R2, R1>) -> R;
}
impl<R1, R2> if_<R1, R2> for Stack<Stack<R2, R1>, bool> {
    fn if_<F, G, R>(self, f: F, g: G) -> R
        where F: FnOnce(Stack<R2, R1>) -> R,
              G: FnOnce(Stack<R2, R1>) -> R,
    {
        let (stack, a) = self.pop();
        if a {
            f(stack)
        } else {
            g(stack)
        }
    }
}

pub trait apply2<A, B, R1, R2> {
    fn apply2<F, R>(self, f: F) -> R
        where F: FnOnce(Stack<R2, R1>, B, A) -> R;
}
impl<A, B, R1, R2> apply2<A, B, R1, R2> for Stack<Stack<Stack<R2, R1>, B>, A> {
    fn apply2<F, R>(self, f: F) -> R
        where F: FnOnce(Stack<R2, R1>, B, A) -> R
    {
        let (stack, a) = self.pop();
        let (stack, b) = stack.pop();
        f(stack, b, a)
    }
}

pub trait swap<A, B, R1, R2> {
    fn swap(self) -> Stack<Stack<Stack<R2, R1>, A>, B>;
}

impl<A, B, R1, R2> swap<A, B, R1, R2> for Stack<Stack<Stack<R2, R1>, B>, A> {
    fn swap(self) -> Stack<Stack<Stack<R2, R1>, A>, B> {
        let (stack, a) = self.pop();
        let (stack, b) = stack.pop();
        let stack = stack.push(a);
        let stack = stack.push(b);
        stack
    }
}

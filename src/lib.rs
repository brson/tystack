#![allow(unused)]

use std::ops::{Add, Sub};
use std::fmt::Debug;

pub struct Stack<B, A>(pub B, pub A);

pub struct End;

impl Stack<End, End> {
    pub fn new() -> Stack<End, End> {
        Stack(End, End)
    }
}

impl<R1, R2> Stack<R2, R1> {
    pub fn push<V>(self, v: V) -> Stack<Stack<R2, R1>, V> {
        Stack(Stack(self.0, self.1), v)
    }
}

impl<A, R1, R2> Stack<Stack<R2, R1>, A> {
    pub fn pop(self) -> (Stack<R2, R1>, A) {
        (self.0, self.1)
    }

    pub fn apply<F, R>(self, f: F) -> R
        where F: FnOnce(Stack<R2, R1>, A) -> R
    {
        let (stack, a) = self.pop();
        f(stack, a)
    }

    pub fn recurse<F>(self, f: F) -> Stack<Stack<R2, R1>, A>
        where F: FnOnce(Stack<Stack<End, End>, A>) -> Stack<Stack<End, End>, A>
    {
        let (stack, a) = self.pop();
        let newstack = Stack::new().push(a);
        let newstack = f(newstack);
        let (newstack, b) = newstack.pop();
        let stack = stack.push(b);
        stack
    }

    pub fn dup(self) -> Stack<Stack<Stack<R2, R1>, A>, A>
        where A: Clone
    {
        self.apply(
            |stack, a| stack.push(a.clone()).push(a))
    }

    pub fn debug(self) -> Stack<Stack<R2, R1>, A>
        where A: Debug
    {
        let (stack, a) = self.pop();
        println!("{:?}", a);
        stack.push(a)
    }
}

impl<R1, R2> Stack<Stack<R2, R1>, bool> {
    pub fn if_<F, G, R>(self, f: F, g: G) -> R
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

impl<A, B, R1, R2> Stack<Stack<Stack<R2, R1>, B>, A> {
    pub fn apply2<F, R>(self, f: F) -> R
        where F: FnOnce(Stack<R2, R1>, B, A) -> R
    {
        let (stack, a) = self.pop();
        let (stack, b) = stack.pop();
        f(stack, b, a)
    }

    pub fn swap(self) -> Stack<Stack<Stack<R2, R1>, A>, B> {
        let (stack, a) = self.pop();
        let (stack, b) = stack.pop();
        let stack = stack.push(a);
        let stack = stack.push(b);
        stack
    }

    pub fn add(self) -> Stack<Stack<R2, R1>, B::Output>
        where B: Add<A>
    {
        self.apply2(
            |stack, b, a| stack.push(b.add(a)))
    }

    pub fn sub(self) -> Stack<Stack<R2, R1>, B::Output>
        where B: Sub<A>
    {
        self.apply2(
            |stack, b, a| stack.push(b.sub(a)))
    }

}

impl<T1, R1, R2> Stack<Stack<Stack<R2, R1>, T1>, T1> {
    pub fn eq(self) -> Stack<Stack<R2, R1>, bool>
        where T1: Eq
    {
        self.apply2(
            |stack, b, a| stack.push(b.eq(&a)))
    }
}

impl<R1, R2> Stack<Stack<Stack<R2, R1>, bool>, bool> {
    pub fn or(self) -> Stack<Stack<R2, R1>, bool> {
        self.apply2(
            |stack, b, a| stack.push(b || a))
    }
}                  

impl<R1, R2> Stack<Stack<R2, R1>, bool> {
    pub fn not(self) -> Stack<Stack<R2, R1>, bool> {
        self.apply(
            |stack, a| stack.push(!a))
    }
}                  

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let stack = Stack((), ());
        let stack = stack.push(1);
        let stack = stack.push(2);
        let stack = stack.dup();
        let stack = stack.swap();
        let (stack, _) = stack.pop();

        let stack = Stack((), ())
            .push(1)
            .push(2)
            .dup()
            .swap();
        let (stack, _) = stack.pop();
    }

}

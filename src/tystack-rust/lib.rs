#![allow(non_camel_case_types)]

extern crate tystack_core as tscore;
#[macro_use]
extern crate tystack_rt as tsrt;

use std::ops::{Add, Sub};
use tscore::Stack;
use tsrt::*;

pub trait clone<A, R1, R2> {
    fn clone(self) -> Stack<Stack<Stack<R2, R1>, A>, A>
        where A: Clone;
}
impl<A, R1, R2> clone<A, R1, R2> for Stack<Stack<R2, R1>, A> {
    fn clone(self) -> Stack<Stack<Stack<R2, R1>, A>, A>
        where A: Clone
    {
        self.apply(
            |stack, a| stack.push(a.clone()).push(a))
    }
}

pub trait add<A, B, R1, R2> {
    fn add(self) -> Stack<Stack<R2, R1>, B::Output>
        where B: Add<A>;
}
impl<A, B, R1, R2> add<A, B, R1, R2> for Stack<Stack<Stack<R2, R1>, B>, A> {
    fn add(self) -> Stack<Stack<R2, R1>, B::Output>
        where B: Add<A>
    {
        self.apply2(
            |stack, b, a| stack.push(b.add(a)))
    }
}

pub trait sub<A, B, R1, R2> {
    fn sub(self) -> Stack<Stack<R2, R1>, B::Output>
        where B: Sub<A>;
}
impl<A, B, R1, R2> sub<A, B, R1, R2> for Stack<Stack<Stack<R2, R1>, B>, A> {
    fn sub(self) -> Stack<Stack<R2, R1>, B::Output>
        where B: Sub<A>
    {
        self.apply2(
            |stack, b, a| stack.push(b.sub(a)))
    }

}

pub trait eq<T1, R1, R2> {
    fn eq(self) -> Stack<Stack<R2, R1>, bool>
        where T1: Eq;
}
impl<T1, R1, R2> eq<T1, R1, R2> for Stack<Stack<Stack<R2, R1>, T1>, T1> {
    fn eq(self) -> Stack<Stack<R2, R1>, bool>
        where T1: Eq
    {
        self.apply2(
            |stack, b, a| stack.push(b.eq(&a)))
    }
}

def_fn_2to1! {
    self or [bool, bool | bool] {
        self.apply2(
            |stack, b, a| stack.push(b || a))
    }        
}

def_fn_1to1! {
    self not [bool | bool] {
        self.apply(
            |stack, a| stack.push(!a))
    }
}

extern crate tystack_core;
extern crate tystack_rt;
extern crate tystack_rust;

use tystack_core::*;
use tystack_rt::*;
use tystack_rust::*;

fn main() {

    let s = Stack::new();
    s.
        push(1).
        clone().
        clone().
        push(1).
        eq().
        swap().
        push(0).
        eq().
        or().
        not().
        debug();
}

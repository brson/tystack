extern crate tystack;

use tystack::Stack;

fn main() {
    let input = 6;
    let stack = Stack::new().push(input);
    let stack = stack.fib();
    let (_, res): (_, u32) = stack.pop();
    println!("fib({}) = {}", input, res);
}

trait Mixins<R2, R1> {
    fn fib(self) -> Stack<Stack<R2, R1>, u32>;
}

impl<R1, R2> Mixins<R2, R1> for Stack<Stack<R2, R1>, u32> {
    fn fib(self) -> Stack<Stack<R2, R1>, u32> {
        self.
            clone().
            clone().
            push(1).
            eq().
            swap().
            push(0).
            eq().
            or().
            not().
            if_(
                |stack| {
                    stack.
                        clone().
                        push(1).
                        sub().
                        recurse(|stack| stack.fib()).
                        swap().
                        push(2).
                        sub().
                        recurse(|stack| stack.fib()).
                        add()
                },
                |stack| stack,
            )
    }
}

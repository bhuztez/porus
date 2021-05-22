#[macro_use]
extern crate porus;
prelude!();

use porus::io::{PeekableSource, Source};
use porus::scan::Consumer;

enum Symbol {
    Operator(u8),
    Operand(isize),
}

use Symbol::Operand;
use Symbol::Operator;

impl Default for Symbol {
    fn default() -> Self {
        Operator(0)
    }
}

impl<'a> Consumer for &'a mut Symbol {
    fn consume<I: Source>(self, s: &mut PeekableSource<I>) -> bool {
        match s.peek() {
            None => false,
            Some(&b'+') => {
                *self = Operator(b'+');
                s.consume();
                true
            }
            Some(&b'-') => {
                *self = Operator(b'-');
                s.consume();
                true
            }
            Some(&b'*') => {
                *self = Operator(b'*');
                s.consume();
                true
            }
            Some(_) => {
                let mut x: isize = 0;
                if Consumer::consume(&mut x, s) {
                    *self = Operand(x);
                    true
                } else {
                    false
                }
            }
        }
    }
}

fn main() {
    let a = &mut Vec::new();

    while let Some(s) = read_opt!() {
        match s {
            Operand(x) => {
                stack::push(a, x);
            }
            Operator(b'+') => {
                let y = stack::pop(a);
                let x = stack::pop(a);
                stack::push(a, x + y);
            }
            Operator(b'-') => {
                let y = stack::pop(a);
                let x = stack::pop(a);
                stack::push(a, x - y);
            }
            Operator(b'*') => {
                let y = stack::pop(a);
                let x = stack::pop(a);
                stack::push(a, x * y);
            }
            Operator(_) => unreachable!(),
        }
    }

    printf!("{:isize}\n", stack::pop(a));
}

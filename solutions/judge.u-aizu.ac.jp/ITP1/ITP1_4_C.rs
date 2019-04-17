#[macro_use]
extern crate porus;
prelude!();

fn main() {
    loop {
        let (mut a, mut op, mut b): (isize, u8, isize) = default();
        read!(&mut a, Char(&mut op), &mut b);

        if op == b'?' {
            break;
        }

        writelnf!(
            "{:d}",
            match op {
                b'+' => a + b,
                b'-' => a - b,
                b'*' => a * b,
                b'/' => a / b,
                _ => panic!(),
            }
        );
    }
}

#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: usize = read!();
    let b: usize = read!();
    let c: usize = read!();
    writelnf!(
        "{:d} {:d} {:d}",
        Ord::min(Ord::min(a, b), c),
        Ord::max(Ord::max(Ord::min(a, b), Ord::min(b, c)), Ord::min(a, c)),
        Ord::max(Ord::max(a, b), c)
    );
}

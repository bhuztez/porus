#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: usize = read!();
    let b: usize = read!();
    let c: usize = read!();
    writelnf!("{:s}", if (a < b) && (b < c) { "Yes" } else { "No" });
}

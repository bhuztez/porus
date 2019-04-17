#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();

    let mut min = isize::max_value();
    let mut max = isize::min_value();
    let mut sum: isize = 0;

    for _ in 0..n {
        let a: isize = read!();

        min = Ord::min(a, min);
        max = Ord::max(a, max);
        sum = sum + a;
    }

    writelnf!("{:d} {:d} {:d}", min, max, sum);
}

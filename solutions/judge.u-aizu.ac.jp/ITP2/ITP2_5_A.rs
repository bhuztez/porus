#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &mut Vec<(isize, isize)> = &mut (0..n)
        .map(|_| {
            let x = read!();
            let y = read!();
            (x, y)
        })
        .collect();

    list::shell_sort(
        a,
        PartialOrd::lt,
        &[
            797161, 265720, 88573, 29524, 9841, 3280, 1093, 364, 121, 40, 13, 4, 1,
        ],
    );

    for (x, y) in list::iter(a) {
        writelnf!("{:d} {:d}", x, y)
    }
}

#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let w: isize = read!();
    let h: isize = read!();
    let x: isize = read!();
    let y: isize = read!();
    let r: isize = read!();
    writelnf!(
        "{:s}",
        if (r <= x) && (x <= (w - r)) && (r <= y) && (y <= (h - r)) {
            "Yes"
        } else {
            "No"
        }
    );
}

#[macro_use]
extern crate porus;
prelude!();

#[derive(Default, Clone, Copy)]
struct Card(u8, u8);

fn stable(b: bool) -> &'static str {
    if b {
        "Stable"
    } else {
        "Not stable"
    }
}

fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);

    let a = &mut vec![Card(0, 0); n];
    for e in a.iter_mut() {
        scanf!(" {:c}{:u8}", &mut e.0, &mut e.1);
    }

    let bi: &mut Vec<usize> = &mut (0..n).collect();
    list::bubble_sort(bi, |&i, &j| list::get(a, i).1 < list::get(a, j).1);
    interleave(
        list::iter(bi),
        || printf!(" "),
        |i| printf!("{:c}{:u}", list::get(a, i).0, list::get(a, i).1),
    );
    printf!(
        "\n{}\n",
        stable(list::is_stable_sort(a, |x, y| x.1 < y.1, bi))
    );

    let si: &mut Vec<usize> = &mut (0..n).collect();
    list::selection_sort(si, |&i, &j| list::get(a, i).1 < list::get(a, j).1);
    interleave(
        list::iter(si),
        || printf!(" "),
        |i| printf!("{:c}{:u}", list::get(a, i).0, list::get(a, i).1),
    );
    printf!(
        "\n{}\n",
        stable(list::is_stable_sort(a, |x, y| x.1 < y.1, si))
    );
}

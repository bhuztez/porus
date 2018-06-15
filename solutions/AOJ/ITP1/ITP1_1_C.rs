#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let stdout = unsafe { &mut STDOUT };
    let (mut a, mut b): (isize, isize) = default();
    read!(&mut a, &mut b);
    printf!(stdout, "%d %d\n", a*b, (a+b)*2);
}

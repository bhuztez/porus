#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let stdout = unsafe { &mut STDOUT };
    let mut n : isize = default();
    read!(&mut n);

    let buf = buffer![];

    for _ in 0..n {
        let mut op : isize = default();
        read!(&mut op);
        if op == 0 {
            let (mut d, mut x) : (isize, isize) = default();
            read!(&mut d, &mut x);
            if d == 0 {
                Deque::push_front(buf, x);
            } else if d == 1 {
                Deque::push_back(buf, x);
            }
        } else if op == 1 {
            let mut p : isize = default();
            read!(&mut p);
            printf!(stdout, "%d\n", buf[p]);
        } else if op == 2 {
            let mut d : isize = default();
            read!(&mut d);
            if d == 0 {
                Deque::pop_front(buf);
            } else if d == 1 {
                Deque::pop_back(buf);
            }
        }
    }
}

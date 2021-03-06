fn main() {
    let mut n = default();
    scanf!("{:usize}", &mut n);
    let s = &mut vec![default(); n];
    for e in List::iter_mut(s) {
        scanf!("{:u}", e);
    }
    let mut q = default();
    scanf!("{:u}", &mut q);

    let mut count: usize = 0;

    for _ in 0..q {
        let mut t = default();
        scanf!("{:u}", &mut t);
        if let Some(_) = List::find(s, &t) {
            count += 1;
        }
    }

    printf!("{:usize}\n", count);
}

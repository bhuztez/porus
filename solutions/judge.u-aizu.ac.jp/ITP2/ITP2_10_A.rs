fn print(x: u32) {
    for i in 0..32usize {
        printf!("{:u}", ((x << i) >> 31) & 1);
    }
    printf!("\n");
}

fn main() {
    let mut x = default();
    scanf!("{:u32}", &mut x);
    print(x);
    print(!x);
    print(x << 1);
    print(x >> 1);
}

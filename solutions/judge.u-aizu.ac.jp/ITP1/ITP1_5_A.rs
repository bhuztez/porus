#[macro_use]
extern crate porus;
prelude!();

fn main() {
    loop {
        let h: u64 = read!();
        let w: u64 = read!();
        if (h == 0) && (w == 0) {
            break;
        }

        for _ in 0..h {
            for _ in 0..w {
                printf!("#");
            }
            printf!("\n");
        }
        printf!("\n");
    }
}

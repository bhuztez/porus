fn main() {
    loop {
        let (mut m, mut f, mut r) = default();
        scanf!("{:i} {:i} {:i}", &mut m, &mut f, &mut r);

        if (m == -1) && (f == -1) && (r == -1) {
            break;
        }

        if (m == -1) || (f == -1) {
            printf!("F\n");
        } else {
            printf!(
                "{}\n",
                match m + f {
                    80..=100 => "A",
                    65..80 => "B",
                    50..65 => "C",
                    30..50 if r >= 50 => "C",
                    30..50 => "D",
                    0..30 => "F",
                    _ => panic!(),
                }
            );
        }
    }
}

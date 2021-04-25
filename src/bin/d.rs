use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        x: i64
    };

    for a in 0..((x as f32).sqrt().sqrt() as i64 + 1) {
        let mut b = a-1;
        while a*a*a*a + b*b*b*b <=x {
            if a*a*a*a*a - b*b*b*b*b == x{
                println!("{} {}", a, b);
                return
            }
            b = b-1;
        }
    }
}

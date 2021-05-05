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
        n: usize,
        k: usize
    };

    let mut v = vec![0;n+1];

    for _ in 0..k{
        input! {
            d: usize,
            a: [usize; d]
        }
        for ai in a{
            v[ai] += 1;
        }
    }

    let ans = v.iter().filter(|&x| x == &0).count() -1;
    println!("{}", ans);
}

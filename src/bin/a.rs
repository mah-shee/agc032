#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
    n: usize,
    mut b:[usize;n],
    }
    let mut ans = Vec::<usize>::new();
    let mut key = true;
    for i in 0..n {
        for j in (0..b.len()).rev() {
            if j + 1 == b[j] {
                ans.push(b[j]);
                b.remove(j);
                break;
            }
        }
        if n - i == b.len() {
            key = false;
        }
    }
    if key {
        for i in ans.iter().rev() {
            println!("{}", i);
        }
    } else {
        println!("-1");
    }
}

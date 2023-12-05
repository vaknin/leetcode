#![allow(dead_code, unused_variables, clippy::ptr_arg, clippy::manual_memcpy)]

use crate::e88::merge;

mod e88;

fn main() {
    let mut nums1 = vec![2,0];
    let mut nums2 = vec![1];
    let m = 1;
    let n = 1;
    merge(&mut nums1, m, &mut nums2, n);
    println!("{nums1:?}");
}

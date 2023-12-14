#![allow(dead_code, unused_variables, clippy::ptr_arg, clippy::manual_memcpy, unused_mut, clippy::collapsible_if)]

use crate::m334::increasing_triplet;

mod m334;

fn main() {

    let nums = vec![5,4,3,2,1];
    let res = increasing_triplet(nums);
    println!("{:?}", res);
}

#![allow(dead_code, unused_variables, clippy::ptr_arg, clippy::manual_memcpy, unused_mut, clippy::collapsible_if)]

use e242::is_anagram;

mod e242;
fn main() {
    let s = "car".to_string();
    let t = "rac".to_string();
    let res = is_anagram(s, t);
    println!("{res}");
    // println!("{nums:?}");
}
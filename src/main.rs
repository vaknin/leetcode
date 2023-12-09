#![allow(dead_code, unused_variables, clippy::ptr_arg, clippy::manual_memcpy, unused_mut, clippy::collapsible_if)]

use crate::e125::is_palindrome;

mod e125;
fn main() {

    let s = "A man, a plan, a canal: Panamaz".to_string();
    let res = is_palindrome(s);
    println!("{res:?}");
}
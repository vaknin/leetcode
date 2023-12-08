use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map: HashSet<i32> = HashSet::with_capacity(nums.len());
    for n in nums.iter() {
        if let false = map.insert(*n) {
            return true;
        }
    }
    false
}
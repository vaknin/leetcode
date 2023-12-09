use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for (i, n) in nums.iter().enumerate() {
        match map.get(&(target - n)) {
            Some(j) => return vec![i as i32,*j as i32],// Found pair
            None => {// Not found - add num to map
                map.insert(*n, i);
            }
        }
    }
    vec![]
}
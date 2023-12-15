pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut min = nums[0];
    let mut max: Option<i32> = None;

    for &n in nums.iter().skip(1) {
        if max.is_some() && n > max.unwrap() {
            return true
        }
        if n < min {
            min = n
        }
        else if n > min {
           max = Some(n) 
        }
    }
    false
}

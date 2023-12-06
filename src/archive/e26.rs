pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

    // Edge - len = 1
    if nums.len() == 1 {
        return 1;
    }

    let mut i = 1;
    while i < nums.len() {
        let prev = nums[i- 1];

        // Duplicate
        if nums[i] == prev {
            nums.remove(i);
            i -= 1;
        }
        i+=1;
    }
    i as i32
}
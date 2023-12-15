pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

    let len = nums.len();
    if len <= 2 {
        return len as i32;
    }

    let mut i = 2;
    let mut u = 2;
    while i < len {
        let three_in_a_row = nums[i] == nums[i-1] && nums[i-1] == nums[i-2];
        let will_not_be_three_in_a_row = nums[i] != nums[u-1] || nums[i] != nums[u-2];

        // Not 3 in a row
        if !three_in_a_row {

            // Different numbers
            if (nums[i] != nums[u] && will_not_be_three_in_a_row) || u == i {
                nums.swap(i, u);
                u+=1;
            } 
        }
        i+=1;
    }
    u as i32
}
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

    // Edge - len = 1
    if nums.len() == 1 {
        return 1;
    }

    let mut i = 1;
    let mut u = 0;
    while i < nums.len() {

        // Not duplicate: place num at 'u'
        if nums[i - 1] != nums[i] {
            nums[u] = nums[i - 1];
            u+=1;
        }
        i+=1;
    }
    nums[u] = nums[i - 1];
    (u+1) as i32
}
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i: i32 = 0;
    while (i as usize) < nums.len() {
        if nums[i as usize] == val {
            nums.remove(i as usize);
            i-=1;
        }
        i+=1;
    }
    i
}
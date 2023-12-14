pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut vec = vec![1; len];
    let mut product = 1;
    for (i, n) in nums.iter().enumerate() {
        vec[i] *= product;
        product *= n;
    }
    
    product = 1;
    for (i, n) in nums.iter().rev().enumerate() {
        vec[len-i-1] *= product;
        product *= n;
    }
    vec
}

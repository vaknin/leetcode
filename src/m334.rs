    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        for (i, n) in nums.iter().enumerate() {
            if nums.get(i+1..).unwrap_or_default().iter().filter(|x| *x>n).count() >= 2 {
                return true            
            }
            // println!("{:?}", a);
        }
        false
    }

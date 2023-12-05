pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

    if m == 0 {
        return nums1.swap_with_slice(nums2);
    }

    if n == 0 {
        return;
    }

    let mut a = m - 1;
    let mut b = n - 1;

    for i in (0..(m + n) as usize).rev() {

        if a == -1 || b == -1{
            if a == -1 {
                for (j, n) in nums2.iter().enumerate().rev() {
                    nums1[j] = *n;
                }
            }
            return;
        }

        if nums1[a as usize] >= nums2[b as usize] {
            nums1.swap(a as usize, i);
            a -= 1;
        }

        else {
            nums1[i] = nums2[b as usize];
            nums2.remove(b as usize);
            b -= 1;
        }
    }
}
def longestOnes(nums: list[int], k: int) -> int:
    zero_indices = [i for (i, n) in enumerate(nums) if n == 0]
    if k >= len(zero_indices):
        return len(nums)

    longest = 0
    iters = len(zero_indices) - k + 1
    for i in range(iters):
        start = zero_indices[i-1] + 1 if i != 0 else 0
        end = zero_indices[i+k] if i+k != len(zero_indices) else None
        vec = nums[start:end]
        if len(vec) > longest:
            longest = len(vec)
    return longest

nums = [0,0,0,0,0,0,1,0] 
k = 1
print(longestOnes(nums, k))

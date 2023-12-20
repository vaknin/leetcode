def longestOnes(nums: list[int], k: int) -> int:
    l, best = 0, 0
    for r, n in enumerate(nums):    
        if n == 0:
            k -= 1
        if k < 0:
            if nums[l] == 0:
                k += 1
            l += 1
        else:
            best = max(best, r-l+1)
    return best
nums = [1,0,1,1,0,0,1,1,0,0,1]
k = 2
longestOnes(nums, k)

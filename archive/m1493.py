def longestSubarray(nums: list[int]) -> int:
    prev, cons, best = 0, 0, 0
    for n in nums:
        if n == 0:
            best = max(best, prev + cons)
            prev = cons
            cons = 0
        else:
            cons += 1
    return min(len(nums) - 1, max(best, prev + cons))

nums = [1,1,1]
print(longestSubarray(nums))

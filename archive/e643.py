def findMaxAverage(nums: list[int], k: int) -> float:
    max_sum = sum(nums[0:k])
    curr_sum = max_sum
    for i in range(1, len(nums) - k + 1):
        curr_sum += nums[i+k-1] - nums[i-1]
        if curr_sum >= max_sum:
            max_sum = curr_sum
    return max_sum / k

nums = [1,12,-5,-6,50,3]
k = 4 
# nums = [1,1,1,1,1,1,2,1,1,1,2]
# k=2
print(findMaxAverage(nums, k))

def pivotIndex(nums: list[int]) -> int:
    tsum = sum(nums)
    csum = 0
    for i, n in enumerate(nums):
        if csum == tsum - (n + csum):
            return i
        csum += n
    return -1

nums = [1,7,3,6,5,6]
print(pivotIndex(nums))

def moveZeroes(nums: list[int]) -> None:
    start: int = None
    end: int = None
    for (i, n) in enumerate(nums):
        if n == 0:
            end = i
            if start is None:
                start = i
        else:
            if start is not None:
                nums[i], nums[start] = nums[start], nums[i]
                if start == end:
                    start = i
                else:
                    start += 1
                end = i

nums = [0,1,0,3,12]
moveZeroes(nums)
print(nums)
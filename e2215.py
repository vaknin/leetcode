def findDifference(nums1: list[int], nums2: list[int]) -> list[list[int]]:
    nums1_set, nums2_set = set(nums1), set(nums2)
    nums1_diff, nums2_diff = [], []
    for n in nums1_set:
        if n not in nums2_set:
            nums1_diff.append(n)
    for n in nums2_set:
        if n not in nums1_set:
            nums2_diff.append(n)
    return [nums1_diff, nums2_diff]
nums1 = [1,2,3]
nums2 = [2,4,6]
print(findDifference(nums1, nums2))

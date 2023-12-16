def maxOperations(nums: list[int], k: int) -> int:
    map = {} 
    operations = 0
    for n in filter(lambda x: x < k, nums):
        if k-n in map: 
            operations += 1
            map[k-n] -= 1
            if map[k-n] == 0:
                map.pop(k-n)
        elif n in map:
            map[n] += 1
        else:
            map[n] = 1
    return operations


nums=[1,2,3,4]
k = 5
print(maxOperations(nums, k))

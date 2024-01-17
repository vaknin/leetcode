def uniqueOccurrences(arr: list[int]) -> bool:
    unique = dict()
    for n in arr:
        curr = 0 if n not in unique else unique[n]
        unique[n] = curr + 1
    return len(set(unique.values())) == len(unique.values())

arr = [1,2,2,1,1,3]
print(uniqueOccurrences(arr))

def maxArea(height: list[int]) -> int:
    i = 0
    j = len(height) - 1
    largest = 0
    while True:
        if i==j:
            break
        largest = max(largest, abs(i-j) * min(height[i], height[j]))
        if height[j] >= height[i]:
            i+=1
        else:
            j-=1
    return largest

height = [1,8,6,2,5,4,8,3,7]
print(maxArea(height))

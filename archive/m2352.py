def equalPairs(grid: list[list[int]]) -> int:
    count = 0
    cols = dict()
    for c in range(len(grid)):
        col = []
        for r in range(len(grid)):
            col.append(grid[r][c])
        cols[c] = col
    for row in grid:
        for c in range(len(grid)):
            if row == cols[c]:
                count+=1
    return count
grid = [[1,1], [1,1]]
print(equalPairs(grid))

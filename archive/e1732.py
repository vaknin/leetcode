def largestAltitude(gain: list[int]) -> int:
    best, alt = 0, 0
    for g in gain:
        alt+=g
        best = max(best, alt)
    return best
gain = [-5,1,5,0,-7]
print(largestAltitude(gain))

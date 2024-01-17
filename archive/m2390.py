def removeStars(s: str) -> str:
    res = ""
    for i, c in enumerate(s):
        if c == '*' and i > 0:
            res = res[:-1]
        else:
            res += c
    return res

s = "leet**cod*e"
print(removeStars(s))

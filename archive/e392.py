def isSubsequence(s: str, t: str) -> bool:
    start = 0
    for c in s:
        index = t.find(c, start)
        if index == -1:
            return False
        start = index + 1
    return True


s = "abc"
t = "ahbgdc"
print(isSubsequence(s, t))
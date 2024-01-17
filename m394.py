def decodeString(s: str) -> str:
    decoded, number, parens, parens_start = "", "", 0, 0
    for i, c in enumerate(s):
        if parens:
            if parens == 1 and c == "]":
                decoded += int(number) * decodeString(s[parens_start:i])
                number, parens, parens_start = "", 0, 0

            elif c == "[":
                parens += 1
            elif c == "]":
                parens -= 1
        else:
            if c.isdigit():
                number += c
            elif c.isalpha():
                decoded += c
            else:
                parens = 1
                parens_start = i+1
    return decoded

s = "3[a]2[bc]"
# s = "abc"
print(decodeString(s))

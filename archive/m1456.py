vowels = {'a', 'e', 'i', 'o', 'u'}
def is_vowel(c: str) -> int:
    return 1 if c in vowels else 0
def maxVowels(s: str, k: int) -> int:
    curr = sum(map(lambda x: is_vowel(x), s[0:k]))
    max_vowels = curr
    for i in range(k, len(s)):
        curr += is_vowel(s[i]) - is_vowel(s[i-k])
        if curr > max_vowels:
            max_vowels = curr
    return max_vowels
s = "abciiidef"
k = 3
print(maxVowels(s, k))

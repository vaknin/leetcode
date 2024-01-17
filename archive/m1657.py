def closeStrings(word1: str, word2: str) -> bool:
    word1_dict, word2_dict = dict(), dict()
    for c in word1:
        curr = 0 if c not in word1_dict else word1_dict[c]
        word1_dict[c] = curr + 1

    for c in word2:
        curr = 0 if c not in word2_dict else word2_dict[c]
        word2_dict[c] = curr + 1

    return word1_dict.keys() == word2_dict.keys() and sorted(word1_dict.values()) == sorted(word2_dict.values())

word1 = "abbzccca"
word2 = "babzzczc"
print(closeStrings(word1, word2))

pub fn merge_alternately(word1: String, word2: String) -> String {
    let (short, long) = if word1.len() <= word2.len() {(word1.clone(), word2.clone())} else {(word2.clone(), word1.clone())};
    word1
        .chars()
        .zip(word2.chars())
        .map(|(a,b)| format!("{a}{b}"))
        .reduce(|acc, e| acc+&e)
        .unwrap()
        +&long[short.len()..]
}
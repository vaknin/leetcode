pub fn is_palindrome(s: String) -> bool {
    let mut filtered: Vec<char>= s.chars().filter(|x| x.is_alphanumeric()).map(|x| x.to_ascii_lowercase()).collect();
    filtered
        .iter()
        .zip(filtered.iter().rev())
        .take(filtered.len() / 2)
        .all(|(a, b)| a==b)
}
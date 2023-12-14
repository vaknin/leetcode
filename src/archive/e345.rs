fn is_vowel(c: char) -> bool { matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') }
pub fn reverse_vowels(s: String) -> String {
    let mut vowel_indices: Vec<usize> = s.char_indices()
        .filter(|(i, c)| is_vowel(*c))
        .map(|(i,c)| i).collect();
    if vowel_indices.len() <= 1 {
        return s;
    }
    let mut s: Vec<char> = s.chars().collect();
    for i in 0..=vowel_indices.len()/2-1 {
        s.swap(vowel_indices[i], vowel_indices[vowel_indices.len()-i-1])
    }
    s.iter().collect()
}
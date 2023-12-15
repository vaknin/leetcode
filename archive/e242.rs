use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_map: HashMap<char, usize> = HashMap::with_capacity(s.len());
    let mut t_map: HashMap<char, usize> = HashMap::with_capacity(t.len());

    for c in s.chars() {
        s_map.entry(c).and_modify(|count| {*count += 1}).or_insert(1);
    }
    for c in t.chars() {
        t_map.entry(c).and_modify(|count| {*count += 1}).or_insert(1);
    }
    s_map == t_map
}
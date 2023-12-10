pub fn is_valid(s: String) -> bool {
    let mut vec: Vec<char> = Vec::with_capacity(s.len());
    for c in s.chars() {
        if ['(', '{', '['].contains(&c) {//Open parenthesis
            vec.push(c);
        }
        else {//Close parenthesis
            if vec.is_empty() { return false };
            let prev = *(vec.last().expect("cannot be empty, we just checked."));
            if (prev == '(' && c == ')') || (prev == '{' && c == '}') || (prev == '[' && c == ']'){
                vec.pop();
            }
            else { return false }
        }
    }
    vec.is_empty()
}
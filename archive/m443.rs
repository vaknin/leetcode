pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut curr = chars[0];
    let mut count = 0;
    let mut i = 0;
    while i < chars.len() {

        // Same char - count and remove
        if chars[i] == curr {
            count+=1;
            if count >= 2 {
                chars.remove(i);
                i -= 1;
            }
            i += 1;
        }

        // Different char, insert the previous curr
        else {
            curr = chars[i];
            let mut skip = 1;
            if count >= 2 {
                for c in count.to_string().chars().rev() {
                    chars.insert(i, c);
                    skip += 1;
                }
            }
            count = 1;
            i += skip;
        }
    }
    if count >= 2 {
        for c in count.to_string().chars() {
            chars.push(c);
        }
    }
    chars.len() as i32
}
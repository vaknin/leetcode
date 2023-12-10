use std::collections::HashSet;

fn calculate_divisors(n: usize) -> Vec<usize> {
    let mut vec = vec![1];
    for i in 2..=(n as f32).sqrt().ceil() as usize {
        if n % i == 0 {
            vec.extend_from_slice(&[i, n/i]);
        }
    }
    vec.dedup();
    vec
}

fn calculate_divisor_strings(str: String) -> HashSet<String>{
    let len = str.len();
    let divisor_numbers = calculate_divisors(len);
    let mut divisor_strings: Vec<String> = Vec::with_capacity(divisor_numbers.len() + 1);
    divisor_strings.push(str.clone());
    
    for d in divisor_numbers {
        if str[0..d].to_string().repeat(len/d) == str {
            divisor_strings.push(str[0..d].to_string());
        }
    }
    divisor_strings.into_iter().collect()
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    calculate_divisor_strings(str1).intersection(&calculate_divisor_strings(str2)).cloned().max().unwrap_or_default()
}
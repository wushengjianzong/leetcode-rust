#![allow(dead_code, unused_imports)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut ret: usize = 0;
        let mut start: usize = 0;
        let mut map: HashMap<char, usize> = HashMap::new();
        for i in 0..chars.len() {
            if let Some(last) = map.get(&chars[i]) {
                for j in start..=*last {
                    map.remove(&chars[j]);
                    start += 1;
                }
            }
            map.insert(chars[i], i);
            ret = if map.len() > ret { map.len() } else { ret };
       }
       return ret as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        println!("{}", Solution::length_of_longest_substring(String::from("abcabcbb")));
        println!("{}", Solution::length_of_longest_substring(String::from("bbbbb")));
        println!("{}", Solution::length_of_longest_substring(String::from("pwwkew")));
        println!("{}", Solution::length_of_longest_substring(String::from("aabab!bb")));
    }
}
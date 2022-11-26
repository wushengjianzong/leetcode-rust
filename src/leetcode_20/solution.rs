#![allow(dead_code, unused_imports)]

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut stack: Vec<char> = vec![];
        for i in 0..chars.len() {
            let top = stack.pop();
            match (top, chars[i]) {
                (None, '(') | (None, '[') | (None, '{') => {
                    stack.push(chars[i]);
                },
                (Some(')'), _) | (Some(']'), _) | (Some('}'), _) => {
                    return false;
                },
                (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') => {
                    continue;
                },
                (Some(prev), '(') | (Some(prev), '[') | (Some(prev), '{') => {
                    stack.push(prev);
                    stack.push(chars[i]);
                },
                _ => {
                    return false;
                }
            }
        }
        return stack.len() == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        println!("{}", Solution::is_valid(String::from("()")));
        println!("{}", Solution::is_valid(String::from("()[]{}")));
        println!("{}", Solution::is_valid(String::from("(]")));
        println!("{}", Solution::is_valid(String::from("{[]}")));
    }
}
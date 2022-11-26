#![allow(dead_code, unused_imports)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ret: i32 = 0;
        let mut i: usize = 0; 
        let chars: Vec<char> = s.chars().collect();
        while i < chars.len() {
            match chars[i] {
                'I' => {
                    ret += 1;
                    if i + 1 < chars.len() {
                        let next: char = chars[i + 1];
                        if next == 'V' || next == 'X' {
                            ret += if next == 'V' { 3 } else { 8 };
                            i += 1;
                        }
                    }
                },
                'X' => {
                    ret += 10;
                    if i + 1 < chars.len() {
                        let next: char = chars[i + 1];
                        if next == 'L' || next == 'C' {
                            ret += if next == 'L' { 30 } else { 80 };
                            i += 1;
                        }
                    }
                },
                'C' => {
                    ret += 100;
                    if i + 1 < chars.len() {
                        let next: char = chars[i + 1];
                        if next == 'D' || next == 'M' {
                            ret += if next == 'D' { 300 } else { 800 };
                            i += 1;
                        }
                    }
                },
                'V'=> {
                    ret += 5;
                }
                'L' => {
                    ret += 50;
                }
                'D' => {
                    ret += 500;
                },
                'M' => {
                    ret += 1000;
                }
                _ => {
                    ret += 0;
                }
            }
            i += 1;
        }
        return  ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        println!("{}", Solution::roman_to_int(String::from("III")));
        println!("{}", Solution::roman_to_int(String::from("IV")));
        println!("{}", Solution::roman_to_int(String::from("IX")));
        println!("{}", Solution::roman_to_int(String::from("IX")));
        println!("{}", Solution::roman_to_int(String::from("LVIII")));
        println!("{}", Solution::roman_to_int(String::from("MCMXCIV")));
        println!("{}", Solution::roman_to_int(String::from("MCDLXXVI")));
    }
}
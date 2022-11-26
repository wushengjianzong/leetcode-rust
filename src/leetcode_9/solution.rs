#![allow(dead_code, unused_imports)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut x: i32 = x;
        if x < 10 {
            return x >= 0;
        }
        if x % 10 == 0 {
            return false;
        }
        let mut y: i32 = 0;
        while x > y {
            y = y * 10 + x % 10;
            x = x / 10;
            println!("x={} y={}", x, y);
        }
        return (x == y) || (x == y / 10);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        println!("{}", Solution::is_palindrome(121));
        println!("{}", Solution::is_palindrome(10));
        println!("{}", Solution::is_palindrome(99));
        println!("{}", Solution::is_palindrome(1221)); 
    }
}
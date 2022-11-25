#![allow(dead_code, unused_imports)]

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;
        let mut prev: Option<i32> = None;
        let mut curr: Option<i32> = None;

        while k <= (nums1.len() + nums2.len()) / 2 {
            if i < nums1.len() && j < nums2.len() {
                if nums1[i] < nums2[j] {
                    prev = curr;
                    curr = Some(nums1[i]);
                    i += 1;
                } else {
                    prev = curr;
                    curr = Some(nums2[j]);
                    j += 1;
                }
            } else {
                if i < nums1.len() {
                    prev = curr;
                    curr = Some(nums1[i]);
                    i += 1;
                } else {
                    prev = curr;
                    curr = Some(nums2[j]);
                    j += 1;
                }
            }
            k += 1;
        }


        match (prev, curr) {
            (None, None) => 0f64,
            (Some(x), None) | (None, Some(x)) => x as f64,
            (Some(x), Some(y)) => {
                if (nums1.len() + nums2.len()) % 2 != 0 { y as f64 } else { (x + y) as f64 / 2f64 }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        println!("{}", Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
        println!("{}", Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
    }
}
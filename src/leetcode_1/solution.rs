use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for j in 0..nums.len() {
            let tmp = target - nums[j];
            if let Some(i) = map.get(&tmp) {
                return vec![*i as i32, j as i32];
            }
            map.insert(nums[j], j);
        }
        return vec![-1, -1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
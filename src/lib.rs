pub mod leetcode_1;
pub mod leetcode_2;
pub mod leetcode_3;
pub mod leetcode_9;
pub mod leetcode_13;
pub mod leetcode_20;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

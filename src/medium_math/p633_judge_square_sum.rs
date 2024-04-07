/**
633. 平方数之和

给定一个非负整数 c ，你要判断是否存在两个整数 a 和 b，使得 a2 + b2 = c 。


https://leetcode.cn/problems/sum-of-square-numbers/description/
*/

pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as i64;
        let mut left: i64 = 0;
        let mut right: i64 = (c as f64).sqrt() as i64 + 1;

        while left <= right {
            let v = left * left + right * right;

            match v.cmp(&c) {
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => return true,
            }
        }

        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::judge_square_sum(5), true);
        assert_eq!(Solution::judge_square_sum(3), false);
    }
    #[test]
    fn test_case() {}
}

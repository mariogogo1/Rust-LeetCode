/**
507. 完美数

对于一个 正整数，如果它和除了它自身以外的所有 正因子 之和相等，我们称它为 「完美数」。

给定一个 整数 n， 如果是完美数，返回 true；否则返回 false。

https://leetcode.cn/problems/perfect-number/description/
*/

pub struct Solution;

use std::f64;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        let mut ans: i32 = 0;
        // +0.5 避免精度問題
        let sqrt_num = (0.5 + num as f64).sqrt() as i32 + 1;
        for i in 1..sqrt_num {
            if num % i == 0 {
                ans += i as i32;
                ans += num / (i as i32);
            };
        }
        ans -= num;
        ans == num
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::check_perfect_number(28), true);
        assert_eq!(Solution::check_perfect_number(7), false);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::check_perfect_number(25), false);
        assert_eq!(Solution::check_perfect_number(6), true);
        assert_eq!(Solution::check_perfect_number(1), false);
    }
}

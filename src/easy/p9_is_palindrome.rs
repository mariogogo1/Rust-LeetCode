/**
9. 回文数

给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。

回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。

例如，121 是回文，而 123 不是。

Hint:

-2^31 <= x <= 2^31 - 1

https://leetcode.cn/problems/palindrome-number/description/
*/

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        };
        let mut xx = x;
        let mut rev_x: i32 = 0;
        while xx > 0 {
            rev_x *= 10;
            rev_x += xx % 10;
            xx /= 10;
        }
        x == rev_x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(0), true);
    }
    #[test]
    fn test_case() {}
}

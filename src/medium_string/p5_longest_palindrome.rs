/**
5. 最长回文子串

给你一个字符串 s，找到 s 中最长的回文子串。

https://leetcode.cn/problems/longest-palindromic-substring/
*/
/// Manacher算法 O(N)
///
/// 可視化：http://manacher-viz.s3-website-us-east-1.amazonaws.com/#/

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        unimplemented!();
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

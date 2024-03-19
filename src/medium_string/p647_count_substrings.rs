/**
647. 回文子串

给你一个字符串 s ，请你统计并返回这个字符串中 回文子串 的数目。

回文字符串 是正着读和倒过来读一样的字符串。

子字符串 是字符串中的由连续字符组成的一个序列。

具有不同开始位置或结束位置的子串，即使是由相同的字符组成，也会被视作不同的子串。

https://leetcode.cn/problems/palindromic-substrings/description/
*/
/// Manacher算法 O(N)
///
/// 可視化：http://manacher-viz.s3-website-us-east-1.amazonaws.com/#/

pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        unimplemented!();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4, 8], 6));
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
    }
}

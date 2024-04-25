/**
516. 最长回文子序列

给你一个字符串 s ，找出其中最长的回文子序列，并返回该序列的长度。

子序列定义为：不改变剩余字符顺序的情况下，删除某些字符或者不删除任何字符形成的一个序列。

https://leetcode.cn/problems/longest-palindromic-subsequence/description/
*/

pub struct Solution;
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![1; s.len()];

        for i in 0..s.len() {
            let mut pre = 0;
            for j in (0..i).rev() {
                if s[j] == s[i] {
                    (dp[j], pre) = (pre + 2, dp[j]);
                } else {
                    (dp[j], pre) = (dp[j].max(dp[j + 1]), dp[j]);
                }
            }
        }
        dp[0]
    }
}

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
        let n = s.len();
        if n == 0 {
            return String::new();
        }

        let mut new_s: Vec<char> = Vec::new();
        new_s.push('@');
        new_s.push('#');
        for ch in s.chars() {
            new_s.push(ch);
            new_s.push('#');
        }
        new_s.push('$');

        let mut arm_len: Vec<usize> = Vec::new();
        let mut right = 0 as usize;
        let mut j = 0 as usize;
        let mut start = 1 as usize;
        let mut end = 0 as usize;
        arm_len.push(0);

        for i in 1..(new_s.len() - 1) {
            //計算回文半徑 並記錄
            let mut cur_arm_len = 0 as usize;
            if right >= i {
                let i_sym = 2 * j - i;
                let min_arm_len = arm_len[i_sym].min(right - i);
                cur_arm_len = Self::expand(&new_s, i - min_arm_len, i + min_arm_len);
            } else {
                cur_arm_len = Self::expand(&new_s, i, i);
            }
            arm_len.push(cur_arm_len);

            if i + cur_arm_len > right {
                j = i;
                right = i + cur_arm_len;
            }
            if cur_arm_len * 2 + 1 + start > end {
                start = i - cur_arm_len;
                end = i + cur_arm_len;
            }
        }

        let mut ans = String::new();
        for i in start..=end {
            if new_s[i] != '#' {
                ans.push(new_s[i]);
            }
        }

        ans
    }
    // 檢查回文半徑是否變大
    fn expand(s: &Vec<char>, mut left: usize, mut right: usize) -> usize {
        while left > 0 && right < (s.len() - 1) && s[left] == s[right] {
            left -= 1;
            right += 1;
        }
        (right - left - 2) / 2
    }
}

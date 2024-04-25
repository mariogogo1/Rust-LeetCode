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
        let n = s.len();

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
        let mut ans = 0 as usize;

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
            ans += (cur_arm_len + 1) / 2;

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
        //println!("{:?}",arm_len);

        ans as i32
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

/**
100294. 统计特殊字母的数量 I

给你一个字符串 word。如果 word 中同时存在某个字母的小写形式和大写形式，则称这个字母为 特殊字母 。

返回 word 中 特殊字母 的数量。

https://leetcode.cn/problems/count-the-number-of-special-characters-i/description/
*/

pub struct Solution;
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut ans = 0;
        let mut char_vec = vec![0; 26];
        let mut char_upper_vec = vec![0; 26];
        for ch in word.chars() {
            if ch.is_ascii_lowercase() {
                char_vec[(ch as u8 - b'a') as usize] += 1;
            } else {
                let index = (ch as u8 - b'A') as usize;
                char_upper_vec[index] += 1;
            }
        }

        for i in 0..char_upper_vec.len() {
            if char_upper_vec[i] > 0 && char_vec[i] > 0 {
                ans += 1;
            }
        }

        ans
    }
}

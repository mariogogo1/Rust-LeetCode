/**
100291. 统计特殊字母的数量 II

给你一个字符串 word。如果 word 中同时出现某个字母 c 的小写形式和大写形式，并且 每个 小写形式的 c 都出现在第一个大写形式的 c 之前，则称字母 c 是一个 特殊字母 。

返回 word 中 特殊字母 的数量。

https://leetcode.cn/problems/count-the-number-of-special-characters-ii/description/
*/
/// Manacher算法 O(N)
///
/// 可視化：http://manacher-viz.s3-website-us-east-1.amazonaws.com/#/

pub struct Solution;
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut ans = 0;
        let mut char_vec = vec![0; 26];
        let mut char_upper_vec = vec![0; 26];
        for ch in word.chars() {
            if ch.is_ascii_lowercase() {
                let index = (ch as u8 - b'a') as usize;

                if char_upper_vec[index] > 0 && char_vec[index] > 0 {
                    char_vec[index] = -1;
                    ans -= 1;
                } else if char_upper_vec[index] == 0 && char_vec[index] == 0 {
                    char_vec[index] += 1;
                }
            } else {
                let index = (ch as u8 - b'A') as usize;
                if char_upper_vec[index] == 0 && char_vec[index] > 0 {
                    ans += 1;
                }
                char_upper_vec[index] += 1;
            }
        }

        ans
    }
}

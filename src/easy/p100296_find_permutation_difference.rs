/**
100296. 两个字符串的排列差

给你两个字符串 s 和 t，每个字符串中的字符都不重复，且 t 是 s 的一个排列。

排列差 定义为 s 和 t 中每个字符在两个字符串中位置的绝对差值之和。

返回 s 和 t 之间的 排列差 。
https://leetcode.cn/problems/permutation-difference-between-two-strings/description/
*/

pub struct Solution;
impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let n = s.len();
        let mut pos: Vec<usize> = vec![0; 26];

        for (i, ch) in s.chars().into_iter().enumerate() {
            pos[(ch as u8 - b'a') as usize] = i;
        }

        let mut ans = 0 as usize;

        for (i, ch) in t.chars().into_iter().enumerate() {
            let idx_add_1 = pos[(ch as u8 - b'a') as usize];
            if idx_add_1 >= i {
                ans += idx_add_1 - i;
            } else {
                ans += i - idx_add_1;
            }
        }

        ans as i32
    }
}

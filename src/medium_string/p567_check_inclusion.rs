/**
567. 字符串的排列

给你两个字符串 s1 和 s2 ，写一个函数来判断 s2 是否包含 s1 的排列。如果是，返回 true ；否则，返回 false 。

换句话说，s1 的排列之一是 s2 的 子串 。

https://leetcode.cn/problems/permutation-in-string/description/
*/

pub struct Solution;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s = s2.as_bytes();
        let p = s1.as_bytes();
        let (s_len, p_len) = (s.len(), p.len());
        if s_len < p_len {
            return false;
        }

        let mut s_count = [0; 26];
        let mut p_count = [0; 26];
        for i in 0..p_len {
            s_count[(s[i] - b'a') as usize] += 1;
            p_count[(p[i] - b'a') as usize] += 1;
        }
        if s_count == p_count {
            return true;
        }

        for i in 0..s_len - p_len {
            s_count[(s[i] - b'a') as usize] -= 1;
            s_count[(s[i + p_len] - b'a') as usize] += 1;
            if s_count == p_count {
                return true;
            }
        }
        false
    }
}

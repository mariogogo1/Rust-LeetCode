/**
438. 找到字符串中所有字母异位词

给定两个字符串 s 和 p，找到 s 中所有 p 的 异位词 的子串，返回这些子串的起始索引。不考虑答案输出的顺序。

异位词 指由相同字母重排列形成的字符串（包括相同的字符串）。

https://leetcode.cn/problems/find-all-anagrams-in-a-string/description/
*/

pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut ans = vec![];
        let mut p_hash_map = [0; 26];
        for c in p.chars() {
            p_hash_map[(c as u8 - b'a') as usize] += 1;
        }
        let mut valid = 0;
        for &count in &p_hash_map {
            if count > 0 {
                valid += 1;
            }
        }
        let mut p_valid = 0;
        let mut s_hash_map = [0; 26];
        let mut start = 0;
        let mut s_vec: Vec<char> = s.chars().collect::<Vec<char>>();
        for (i, &c) in s_vec.iter().enumerate() {
            let idx = (c as u8 - b'a') as usize;
            if p_hash_map[idx] > 0 {
                s_hash_map[idx] += 1;
                if s_hash_map[idx] == p_hash_map[idx] {
                    p_valid += 1;
                } else if s_hash_map[idx] == p_hash_map[idx] + 1 {
                    p_valid -= 1;
                }
            } else {
                s_hash_map = [0; 26];
                p_valid = 0;
                start = i + 1;
            }
            if i - p.len() as usize == start {
                let start_char = s_vec[start];
                let start_idx = (start_char as u8 - b'a') as usize;
                if p_hash_map[start_idx] > 0 {
                    s_hash_map[start_idx] -= 1;
                    if s_hash_map[start_idx] == p_hash_map[start_idx] {
                        p_valid += 1;
                    } else if s_hash_map[start_idx] == p_hash_map[start_idx] - 1 {
                        p_valid -= 1;
                    }
                }
                start += 1;
            }
            if p_valid == valid {
                ans.push(start as i32);
            }
        }
        ans
    }
}

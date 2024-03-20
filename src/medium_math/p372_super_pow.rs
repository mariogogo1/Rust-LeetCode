/**
3. 无重复字符的最长子串

给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串的长度。

https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/
*/

pub struct Solution;

use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut index_hashmap: HashMap<char, usize> = HashMap::new();
        let mut dp_vec: Vec<i32> = vec![0; s.len() + 1];
        let mut ans: i32 = 0;
        for (i, x) in s.chars().enumerate() {
            // 原字串需要borrow
            match index_hashmap.get(&x) {
                Some(&value) => dp_vec[i + 1] = min(dp_vec[i] + 1, (i - value) as i32),
                None => dp_vec[i + 1] = dp_vec[i] + 1,
            };
            index_hashmap.insert(x, i);
            ans = max(ans, dp_vec[i + 1])
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }
    #[test]
    fn test_case() {}
}

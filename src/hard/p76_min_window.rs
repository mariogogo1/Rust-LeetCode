/**
76. 最小覆盖子串

给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 "" 。



注意：

对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
如果 s 中存在这样的子串，我们保证它是唯一的答案。

https://leetcode.cn/problems/minimum-window-substring/description/
*/
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::new();
        }

        let mut window: HashMap<u8, i32> = HashMap::new();
        let mut need: HashMap<u8, i32> = HashMap::new();
        let mut valid = 0;
        let (mut left, mut right) = (0, 0);
        let (mut start, mut end) = (0, 0);

        for c in t.bytes() {
            *need.entry(c).or_insert(0) += 1;
        }

        while right < s.len() {
            let in_char = s.as_bytes()[right];
            right += 1;
            if let Some(count) = need.get(&in_char) {
                window.entry(in_char).and_modify(|e| *e += 1).or_insert(1);
                if window[&in_char] == *count {
                    valid += 1;
                }
            }

            while valid == need.len() {
                if end == 0 || right - left < end - start {
                    start = left;
                    end = right;
                }
                let out_char = s.as_bytes()[left];
                left += 1;
                if let Some(count) = need.get(&out_char) {
                    if let Some(e) = window.get_mut(&out_char) {
                        if *e == *count {
                            valid -= 1;
                        }
                        *e -= 1;
                    }
                }
            }
        }

        s[start..end].to_string()
    }
}

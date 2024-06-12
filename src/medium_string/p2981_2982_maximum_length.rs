/**
2981. 找出出现至少三次的最长特殊子字符串 I
2982. 找出出现至少三次的最长特殊子字符串 II

给你一个仅由小写英文字母组成的字符串 s 。

如果一个字符串仅由单一字符组成，那么它被称为 特殊 字符串。例如，字符串 "abc" 不是特殊字符串，而字符串 "ddd"、"zz" 和 "f" 是特殊字符串。

返回在 s 中出现 至少三次 的 最长特殊子字符串 的长度，如果不存在出现至少三次的特殊子字符串，则返回 -1 。

子字符串 是字符串中的一个连续 非空 字符序列。

https://leetcode.cn/problems/find-longest-special-substring-that-occurs-thrice-i/description/
https://leetcode.cn/problems/find-longest-special-substring-that-occurs-thrice-ii/description/
*/

pub struct Solution;

use std::mem::swap;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut length_vec = vec![vec![0; 3]; 26];
        let mut cnt = 0;
        let s = s.as_bytes();

        for idx in 0..s.len() {
            cnt += 1;
            if idx + 1 == s.len() || s[idx] != s[idx + 1] {
                let ch = (s[idx] - b'a') as usize;
                if length_vec[ch][0] <= cnt {
                    swap(&mut length_vec[ch][0], &mut cnt);
                }
                if length_vec[ch][1] <= cnt {
                    swap(&mut length_vec[ch][1], &mut cnt);
                }
                if length_vec[ch][2] <= cnt {
                    swap(&mut length_vec[ch][2], &mut cnt);
                }
                cnt = 0;
            }
        }

        let mut ans = 0;
        for length in length_vec {
            if length[0] == 0 {
                continue;
            }
            ans = ans
                .max(length[0] - 2)
                .max(length[1].min(length[0] - 1))
                .max(length[2]);
        }
        if ans > 0 {
            return ans;
        }
        -1
    }
}

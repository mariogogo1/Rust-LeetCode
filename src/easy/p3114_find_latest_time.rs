/**
3114. 替换字符可以得到的最晚时间

给你一个字符串 s，表示一个 12 小时制的时间格式，其中一些数字（可能没有）被 "?" 替换。

12 小时制时间格式为 "HH:MM" ，其中 HH 的取值范围为 00 至 11，MM 的取值范围为 00 至 59。最早的时间为 00:00，最晚的时间为 11:59。

你需要将 s 中的 所有 "?" 字符替换为数字，使得结果字符串代表的时间是一个 有效 的 12 小时制时间，并且是可能的 最晚 时间。

返回结果字符串。
https://leetcode.cn/problems/latest-time-you-can-obtain-after-replacing-characters/description/
*/

pub struct Solution;

impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut ans = vec![];
        let x = s.chars().collect::<Vec<_>>();

        if x[0] == '?' && x[1] == '?' {
            ans.push('1');
            ans.push('1');
        } else if x[0] == '?' && x[1] != '?' {
            if x[1] == '0' || x[1] == '1' {
                ans.push('1');
            } else {
                ans.push('0');
            }
            ans.push(x[1]);
        } else if x[0] != '?' && x[1] == '?' {
            ans.push(x[0]);
            if x[0] == '0' {
                ans.push('9');
            } else {
                ans.push('1');
            }
        } else {
            ans.push(x[0]);
            ans.push(x[1]);
        }

        ans.push(':');

        if x[3] == '?' {
            ans.push('5');
        } else {
            ans.push(x[3]);
        }

        if x[4] == '?' {
            ans.push('9');
        } else {
            ans.push(x[4]);
        }

        ans.into_iter().collect()
    }
}

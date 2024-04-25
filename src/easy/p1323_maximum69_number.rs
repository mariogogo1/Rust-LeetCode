/**
1323. 6 和 9 组成的最大数字
给你一个仅由数字 6 和 9 组成的正整数 num。

你最多只能翻转一位数字，将 6 变成 9，或者把 9 变成 6 。

请返回你可以得到的最大数字。

https://leetcode.cn/problems/maximum-69-number/description/
*/

pub struct Solution;

impl Solution {
    pub fn maximum69_number(mut num: i32) -> i32 {
        let mut rev_t = false;
        let mut rev = 0;

        while num > 0 {
            rev *= 10;
            rev += num % 10;
            num /= 10;
        }

        while rev > 0 {
            num *= 10;
            if !rev_t {
                if rev % 10 == 6 {
                    num += 9;
                    rev_t = true;
                } else {
                    num += rev % 10;
                }
            } else {
                num += rev % 10;
            }
            rev /= 10;
        }
        num
    }
}

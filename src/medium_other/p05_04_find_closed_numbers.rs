/**
面试题 05.04. 下一个数

下一个数。给定一个正整数，找出与其二进制表达式中1的个数相同且大小最接近的那两个数（一个略大，一个略小）

https://leetcode.cn/problems/closed-number-lcci/description/
*/

/// 一般的做法  找最大值：找盡量最右邊的1，向左移動 ，如果都不能移動才回傳-1；找最小值，找盡量最右邊的1，向右移動，如果都不能移動才回傳-1
/// 特殊的作法：Gosper's Hack 是一種高效的算法，用於在二進制表示中找到下一個有相同位元數量的數字。
pub struct Solution;

impl Solution {
    pub fn find_closed_numbers(num: i32) -> Vec<i32> {
        let num: i64 = num as i64;
        let mut big: i64 = num;
        let mut small = (1 << 32) - 1 - num;

        // 大
        let rb = big & -big;
        let cb = big + rb;
        big = (((cb ^ big) >> 2) / rb) | cb;
        if big < num || big > 1 << 31 {
            big = -1;
        }

        // 小
        let rs = small & -small;
        let cs = small + rs;
        small = (((cs ^ small) >> 2) / rs) | cs;
        small = (1 << 32) - 1 - small;
        if small > num || small <= 0 {
            small = -1;
        }

        vec![big as i32, small as i32]
    }
}

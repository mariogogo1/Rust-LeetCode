/**
3133. 数组最后一个元素的最小值

给你两个整数 n 和 x 。你需要构造一个长度为 n 的 正整数 数组 nums ，对于所有 0 <= i < n - 1 ，满足 nums[i + 1] 大于 nums[i] ，并且数组 nums 中所有元素的按位 AND 运算结果为 x 。

返回 nums[n - 1] 可能的 最小 值。

https://leetcode.cn/problems/minimum-array-end/description/
*/
pub struct Solution;

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut n = (n - 1) as i64;
        let mut x = x as i64;

        let mut idx = 0;
        while n > 0 {
            let one = n & 1;
            loop {
                if x >> idx & 1 == 0 {
                    break;
                }
                idx += 1;
            }
            x |= one << idx;

            n >>= 1;
            idx += 1;
        }

        x
    }
}

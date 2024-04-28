/**
995. K 连续位的最小翻转次数

给定一个二进制数组 nums 和一个整数 k 。

k位翻转 就是从 nums 中选择一个长度为 k 的 子数组 ，同时把子数组中的每一个 0 都改成 1 ，把子数组中的每一个 1 都改成 0 。

返回数组中不存在 0 所需的最小 k位翻转 次数。如果不可能，则返回 -1 。

子数组 是数组的 连续 部分。
https://leetcode.cn/problems/minimum-number-of-k-consecutive-bit-flips/description/
*/
pub struct Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut cnt_vecdeque: VecDeque<usize> = VecDeque::new();
        let mut ans = 0;

        for i in 0..nums.len() {
            if let Some(&x) = cnt_vecdeque.front() {
                if x + k <= i {
                    cnt_vecdeque.pop_front();
                }
            }
            if cnt_vecdeque.len() % 2 == nums[i] as usize {
                if i > nums.len() - k {
                    return -1;
                }
                cnt_vecdeque.push_back(i);
                ans += 1;
            }
        }

        ans
    }
}

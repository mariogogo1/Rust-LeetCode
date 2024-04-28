/**
2233. K 次增加后的最大乘积

给你一个非负整数数组 nums 和一个整数 k 。每次操作，你可以选择 nums 中 任一 元素并将它 增加 1 。

请你返回 至多 k 次操作后，能得到的 nums的 最大乘积 。由于答案可能很大，请你将答案对 109 + 7 取余后返回。

https://leetcode.cn/problems/maximum-product-after-k-increments/description/
*/

pub struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// 每次挑選最小的數字進行操作，K次後全部相乘
impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        let mut min_heap = BinaryHeap::new();
        for num in nums {
            min_heap.push(Reverse(num));
        }

        for i in 0..k {
            if let Some(Reverse(val)) = min_heap.pop() {
                min_heap.push(Reverse(val + 1));
            }
        }

        let mut ans = 1 as i64;
        for Reverse(num) in min_heap {
            ans *= num as i64;
            ans %= Self::VAL_MOD;
        }

        ans as i32
    }
}

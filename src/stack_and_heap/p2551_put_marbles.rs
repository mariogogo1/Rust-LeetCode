/**
2551. 将珠子放入背包中

你有 k 个背包。给你一个下标从 0 开始的整数数组 weights ，其中 weights[i] 是第 i 个珠子的重量。同时给你整数 k 。

请你按照如下规则将所有的珠子放进 k 个背包。

没有背包是空的。
如果第 i 个珠子和第 j 个珠子在同一个背包里，那么下标在 i 到 j 之间的所有珠子都必须在这同一个背包中。
如果一个背包有下标从 i 到 j 的所有珠子，那么这个背包的价格是 weights[i] + weights[j] 。
一个珠子分配方案的 分数 是所有 k 个背包的价格之和。

请你返回所有分配方案中，最大分数 与 最小分数 的 差值 为多少。

https://leetcode.cn/problems/put-marbles-in-bags/description/
*/

pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let mut max_heap = BinaryHeap::new();
        let mut min_heap = BinaryHeap::new();
        let mut max_sum: i64 = 0;
        let mut min_sum: i64 = 0;
        for i in 1..weights.len() {
            max_heap.push(weights[i] + weights[i - 1]);
            min_heap.push(Reverse(weights[i] + weights[i - 1]));
        }
        for _ in 0..(k - 1) {
            if let Some(val) = max_heap.pop() {
                max_sum += val as i64;
            }
            if let Some(Reverse(val)) = min_heap.pop() {
                min_sum += val as i64;
            }
        }

        max_sum - min_sum
    }
}

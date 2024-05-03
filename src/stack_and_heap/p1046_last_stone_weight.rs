/**
1046. 最后一块石头的重量

有一堆石头，每块石头的重量都是正整数。

每一回合，从中选出两块 最重的 石头，然后将它们一起粉碎。假设石头的重量分别为 x 和 y，且 x <= y。那么粉碎的可能结果如下：

如果 x == y，那么两块石头都会被完全粉碎；
如果 x != y，那么重量为 x 的石头将会完全粉碎，而重量为 y 的石头新重量为 y-x。
最后，最多只会剩下一块石头。返回此石头的重量。如果没有石头剩下，就返回 0。

https://leetcode.cn/problems/last-stone-weight/description/
*/

pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut max_heap = BinaryHeap::new();
        for i in 0..stones.len() {
            max_heap.push(stones[i]);
        }
        while let Some(stone_a) = max_heap.pop() {
            if let Some(stone_b) = max_heap.pop() {
                if stone_a > stone_b {
                    max_heap.push(stone_a - stone_b);
                }
            } else {
                return stone_a;
            }
        }
        0
    }
}

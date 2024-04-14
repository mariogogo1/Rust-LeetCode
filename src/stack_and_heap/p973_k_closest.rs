/**
973. 最接近原点的 K 个点

给定一个数组 points ，其中 points[i] = [xi, yi] 表示 X-Y 平面上的一个点，并且是一个整数 k ，返回离原点 (0,0) 最近的 k 个点。

这里，平面上两点之间的距离是 欧几里德距离（ √(x1 - x2)2 + (y1 - y2)2 ）。

你可以按 任何顺序 返回答案。除了点坐标的顺序之外，答案 确保 是 唯一 的。

https://leetcode.cn/problems/k-closest-points-to-origin/description/
*/

pub struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut max_heap = BinaryHeap::new();
        for point in points {
            max_heap.push(Reverse((point[0] * point[0] + point[1] * point[1], point)));
        }
        let mut ans = Vec::new();
        for i in 0..k {
            if let Some(Reverse(tuple)) = max_heap.pop() {
                ans.push(tuple.1);
            }
        }

        ans
    }
}

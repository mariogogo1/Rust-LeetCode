/**
2386. 找出数组的第 K 大和

给你一个整数数组 nums 和一个 正 整数 k 。你可以选择数组的任一 子序列 并且对其全部元素求和。

数组的 第 k 大和 定义为：可以获得的第 k 个 最大 子序列和（子序列和允许出现重复）

返回数组的 第 k 大和 。

子序列是一个可以由其他数组删除某些或不删除元素派生而来的数组，且派生过程不改变剩余元素的顺序。

注意：空子序列的和视作 0 。

https://leetcode.cn/problems/find-the-k-sum-of-an-array/description/
*/
/// 這題很關鍵，幾乎是遞增子序列的基礎題目，俄羅斯套娃信封這種經典題目就是從這裡延伸的
pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// 這題知識點有兩個
/// 1. 原問題等價於 找出絕對值数组的第 K 小和
///   最大值 = 數組的所有正數之和
///   找出次大的子序列和就是需要"最大值"-正數 或是 "最大值"+負數
///   ="最大值"-|正數或負數|
///   -> 找|正數或負數|的最小值
/// 2. 維護子序列小根堆的方法
///    小根堆堆頂被彈出之後，要加入兩個新的元素，這樣才能紀錄子序列的最小值
///    例如：小根堆 BinaryHeap<(序列和,序列最後元素的下標)>  ，元素數列 [3,4,5]
///    堆頂元素是(3+4=7,1),當7被彈出之後，加入兩個新的元素 (7-4+5=8,2) 跟 (7+5=12,2)

impl Solution {
    pub fn k_sum(mut nums: Vec<i32>, k: i32) -> i64 {
        let mut max_sum: i64 = 0;
        for num in nums.iter_mut() {
            if *num > 0 {
                max_sum = max_sum + *num as i64;
            } else {
                *num *= -1;
            }
        }
        if k == 1 {
            return max_sum;
        }

        nums.sort_unstable();

        let mut min_heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        min_heap.push(Reverse((nums[0] as i64, 0 as usize)));

        for i in 0..(k - 2) {
            if let Some(Reverse(k_abs_sum)) = min_heap.pop() {
                let idx = k_abs_sum.1;
                if idx < nums.len() - 1 {
                    min_heap.push(Reverse((
                        k_abs_sum.0 - nums[idx] as i64 + nums[idx + 1] as i64,
                        idx + 1,
                    )));
                    min_heap.push(Reverse((k_abs_sum.0 + nums[idx + 1] as i64, idx + 1)));
                }
            }
        }

        if let Some(Reverse(k_abs_sum)) = min_heap.peek() {
            return max_sum - k_abs_sum.0;
        }
        0
    }
}

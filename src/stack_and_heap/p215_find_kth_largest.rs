/**
215. 数组中的第K个最大元素

给定整数数组 nums 和整数 k，请返回数组中第 k 个最大的元素。

请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

你必须设计并实现时间复杂度为 O(n) 的算法解决此问题。

https://leetcode.cn/problems/kth-largest-element-in-an-array/description/
*/

pub struct Solution;
use std::collections::BinaryHeap;
impl Solution {
    /// 題目的數據量很小 可以開一個陣列紀錄數字出現的次數
    pub fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut record = vec![0; 20001];
        // 將數字記錄到對應的位置 1 記錄到 下標10001的位置
        for num in nums.iter().map(|x| x + 10000) {
            record[num as usize] += 1;
        }

        let mut ans = 0;
        for idx in (0..record.len()).rev() {
            if k > 0 {
                k -= record[idx];
            } else {
                ans = (idx as i32) - 9999;
                break;
            }
        }

        ans
    }
    /// 大根堆
    pub fn find_kth_largest_heap(nums: Vec<i32>, k: i32) -> i32 {
        let mut max_heap = BinaryHeap::new();
        for i in 0..nums.len() {
            max_heap.push(nums[i]);
        }
        for i in 0..(k - 1) {
            max_heap.pop();
        }
        if let Some(ans) = max_heap.pop() {
            return ans;
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case() {}
}

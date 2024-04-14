/**
347. 前 K 个高频元素

给你一个整数数组 nums 和一个整数 k ，请你返回其中出现频率前 k 高的元素。你可以按 任意顺序 返回答案。

提示：

1 <= nums.length <= 105
k 的取值范围是 [1, 数组中不相同的元素的个数]
题目数据保证答案唯一，换句话说，数组中前 k 个高频元素的集合是唯一的


进阶：你所设计算法的时间复杂度 必须 优于 O(n log n) ，其中 n 是数组大小。

https://leetcode.cn/problems/top-k-frequent-elements/description/
*/

pub struct Solution;

use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut fre_hashmap: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let count = fre_hashmap.entry(nums[i]).or_insert(0);
            *count += 1;
        }

        let mut max_heap = BinaryHeap::new();
        for (k, v) in fre_hashmap {
            max_heap.push((v, k));
        }

        let mut ans = Vec::new();
        for i in 0..k {
            if let Some(num) = max_heap.pop() {
                ans.push(num.1);
            }
        }

        ans
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

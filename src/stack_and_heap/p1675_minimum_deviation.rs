/**
1675. 数组的最小偏移量

给你一个由 n 个正整数组成的数组 nums 。

你可以对数组的任意元素执行任意次数的两类操作：

如果元素是 偶数 ，除以 2
例如，如果数组是 [1,2,3,4] ，那么你可以对最后一个元素执行此操作，使其变成 [1,2,3,2]
如果元素是 奇数 ，乘上 2
例如，如果数组是 [1,2,3,4] ，那么你可以对第一个元素执行此操作，使其变成 [2,2,3,4]
数组的 偏移量 是数组中任意两个元素之间的 最大差值 。

返回数组在执行某些操作之后可以拥有的 最小偏移量 。
https://leetcode.cn/problems/minimize-deviation-in-array/description/
*/
pub struct Solution;
use std::collections::BinaryHeap;

/// 這題的性質要給嚴格證明有點困難....
/// 作法：
/// 全部變成偶數，維護大根堆跟最小值，
/// 因為在最大值縮小的情況下，答案才有可能更優
/// 所以將堆頂的元素/2，直到不能處理為止。
/// 維護過程中的答案，最小偏移量出現在過程之中
impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut min_num = i32::MAX;
        for i in 0..nums.len() {
            if nums[i] % 2 == 0 {
                max_heap.push(nums[i]);
                min_num = min_num.min(nums[i]);
            } else {
                max_heap.push(nums[i] * 2);
                min_num = min_num.min(nums[i] * 2);
            }
        }

        while let Some(&val) = max_heap.peek() {
            ans = ans.min(val - min_num);
            if val % 2 == 1 {
                break;
            } else {
                max_heap.pop();
                max_heap.push(val / 2);
                min_num = min_num.min(val / 2);
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

/**
560. 和为 K 的子数组

给你一个整数数组 nums 和一个整数 k ，请你统计并返回 该数组中和为 k 的子数组的个数 。

子数组是数组中元素的连续非空序列。

https://leetcode.cn/problems/subarray-sum-equals-k/description/
*/
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix_sum: i32 = 0;
        let mut sum_hashmap: HashMap<i32, i32> = HashMap::new();
        let mut ans: i32 = 0;
        sum_hashmap.insert(0, 1);
        for num in nums {
            prefix_sum += num;
            ans += *sum_hashmap.get(&(prefix_sum - k)).unwrap_or(&0);
            *sum_hashmap.entry(prefix_sum).or_insert(0) += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    }
    #[test]
    fn test_case_1() {
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
    #[test]
    fn test_case_2() {}
}

/**
219. 存在重复元素 II

给你一个整数数组 nums 和一个整数 k ，判断数组中是否存在两个 不同的索引 i 和 j ，满足 nums[i] == nums[j] 且 abs(i - j) <= k 。如果存在，返回 true ；否则，返回 false 。

https://leetcode.cn/problems/contains-duplicate-ii/description/
*/

pub struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut set: HashSet<i32> = HashSet::new();
        for i in 0..nums.len() {
            if i >= 1 + k {
                set.remove(&nums[i - k - 1]);
            }
            if set.contains(&nums[i]) {
                return true;
            }
            set.insert(nums[i]);
        }
        return false;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
    }
    #[test]
    fn test_case() {}
}

/**
217. 存在重复元素

给你一个整数数组 nums 。如果任一值在数组中出现 至少两次 ，返回 true ；如果数组中每个元素互不相同，返回 false 。

https://leetcode.cn/problems/contains-duplicate/description/
*/

pub struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for num in nums {
            if set.contains(&num) {
                return true;
            }
            set.insert(num);
        }
        return false;
    }
}

/**
34. 在排序数组中查找元素的第一个和最后一个位置

给你一个按照非递减顺序排列的整数数组 nums，和一个目标值 target。请你找出给定目标值在数组中的开始位置和结束位置。

如果数组中不存在目标值 target，返回 [-1, -1]。

你必须设计并实现时间复杂度为 O(log n) 的算法解决此问题。

https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/description/
*/
pub struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l: usize = 0;
        let mut r: usize = nums.len();
        let mut first_pos: i32 = 0;
        let mut last_pos: i32 = 0;

        while l < r {
            let m = (l + r) / 2;
            if nums[m] >= target {
                r = m;
            } else if nums[m] < target {
                l = m + 1;
            }
        }
        first_pos = l as i32;

        l = 0;
        r = nums.len();
        while l < r {
            let m = (l + r) / 2;
            if nums[m] > target {
                r = m;
            } else if nums[m] <= target {
                l = m + 1;
            }
        }
        last_pos = l as i32;

        if first_pos == last_pos {
            return vec![-1, -1];
        }
        vec![first_pos, last_pos - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
    }
    #[test]
    fn test_case() {}
}

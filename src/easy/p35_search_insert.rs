/**
35. 搜索插入位置

给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。

请必须使用时间复杂度为 O(log n) 的算法。

https://leetcode.cn/problems/search-insert-position/description/
*/
pub struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = nums.len();
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] == target {
                return m as i32;
            } else if nums[m] > target {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }
    #[test]
    fn test_case() {}
}

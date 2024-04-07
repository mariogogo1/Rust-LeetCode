/**
977. 有序数组的平方

给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
https://leetcode.cn/problems/squares-of-a-sorted-array/description/
*/

pub struct Solution;

impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            nums[i] *= nums[i];
        }
        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;
        let mut i: usize = nums.len() - 1;
        let mut ans: Vec<i32> = vec![0; nums.len()];
        while l < r {
            if nums[l] <= nums[r] {
                ans[i] = nums[r];
                r -= 1;
            } else {
                ans[i] = nums[l];
                l += 1;
            }
            i -= 1;
        }
        ans[0] = nums[l];
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
    #[test]
    fn test_case() {}
}

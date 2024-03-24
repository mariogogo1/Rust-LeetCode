/**
2104. 子数组范围和

给你一个整数数组 nums 。nums 中，子数组的 范围 是子数组中最大元素和最小元素的差值。

返回 nums 中 所有 子数组范围的 和 。

子数组是数组中一个连续 非空 的元素序列。
https://leetcode.cn/problems/sum-of-subarray-minimums/description/
*/
/// 類似 第907題

pub struct Solution;

impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut ans: i64 = 0;
        let mut left_max_index: Vec<usize> = vec![0; n];
        let mut right_max_index: Vec<usize> = vec![n - 1; n];
        let mut stack_max: Vec<usize> = Vec::new();
        let mut left_min_index: Vec<usize> = vec![0; n];
        let mut right_min_index: Vec<usize> = vec![n - 1; n];
        let mut stack_min: Vec<usize> = Vec::new();
        for i in 0..n {
            while let Some(&top) = stack_max.last() {
                if nums[top] <= nums[i] {
                    right_max_index[top] = i - 1;
                    stack_max.pop();
                } else {
                    left_max_index[i] = top + 1;
                    break;
                }
            }
            stack_max.push(i);

            while let Some(&top) = stack_min.last() {
                if nums[top] >= nums[i] {
                    right_min_index[top] = i - 1;
                    stack_min.pop();
                } else {
                    left_min_index[i] = top + 1;
                    break;
                }
            }
            stack_min.push(i);
        }

        for i in 0..n {
            let ln_max = (i - left_max_index[i] + 1) as i64;
            let rn_max = (right_max_index[i] - i + 1) as i64;
            let ln_min = (i - left_min_index[i] + 1) as i64;
            let rn_min = (right_min_index[i] - i + 1) as i64;
            ans += (ln_max * rn_max - ln_min * rn_min) * nums[i] as i64;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::sub_array_ranges(vec![1, 2, 3]), 4);
        assert_eq!(Solution::sub_array_ranges(vec![1, 3, 3]), 4);
        assert_eq!(Solution::sub_array_ranges(vec![4, -2, -3, 4, 1]), 59);
    }
    #[test]
    fn test_case() {}
}

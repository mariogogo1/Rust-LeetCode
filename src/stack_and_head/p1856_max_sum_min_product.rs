/**
1856. 子数组最小乘积的最大值

一个数组的 最小乘积 定义为这个数组中 最小值 乘以 数组的 和 。

比方说，数组 [3,2,5] （最小值是 2）的最小乘积为 2 * (3+2+5) = 2 * 10 = 20 。
给你一个正整数数组 nums ，请你返回 nums 任意 非空子数组 的最小乘积 的 最大值 。由于答案可能很大，请你返回答案对  109 + 7 取余 的结果。

请注意，最小乘积的最大值考虑的是取余操作 之前 的结果。题目保证最小乘积的最大值在 不取余 的情况下可以用 64 位有符号整数 保存。

子数组 定义为一个数组的 连续 部分。
https://leetcode.cn/problems/maximum-subarray-min-product/description/
*/

pub struct Solution;
impl Solution {
    pub const VAL_MOD: i64 = 1_000_000_007;

    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans: i64 = 0;
        let mut left_index: Vec<usize> = vec![0; n];
        let mut right_index: Vec<usize> = vec![n - 1; n];
        let mut prefix_sum: Vec<i64> = vec![0; n + 1];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..n {
            prefix_sum[i + 1] = nums[i] as i64 + prefix_sum[i];
            while let Some(&top) = stack.last() {
                if nums[top] >= nums[i] {
                    right_index[top] = i - 1;
                    stack.pop();
                } else {
                    left_index[i] = top + 1;
                    break;
                }
            }
            stack.push(i);
        }
        for i in 0..n {
            ans = ans
                .max((prefix_sum[right_index[i] + 1] - prefix_sum[left_index[i]]) * nums[i] as i64);
        }
        ans %= Self::VAL_MOD;
        return ans as i32;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::max_sum_min_product(vec![1, 2, 3, 2]), 14);
        assert_eq!(Solution::max_sum_min_product(vec![2, 3, 3, 1, 2]), 18);
        assert_eq!(Solution::max_sum_min_product(vec![3, 1, 5, 6, 4, 2]), 60);
    }
    #[test]
    fn test_case() {}
}

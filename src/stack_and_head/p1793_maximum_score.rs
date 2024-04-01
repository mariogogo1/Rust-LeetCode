/**
1793. 好子数组的最大分数

给你一个整数数组 nums （下标从 0 开始）和一个整数 k 。

一个子数组 (i, j) 的 分数 定义为 min(nums[i], nums[i+1], ..., nums[j]) * (j - i + 1) 。一个 好 子数组的两个端点下标需要满足 i <= k <= j 。

请你返回 好 子数组的最大可能 分数 。

https://leetcode.cn/problems/maximum-score-of-a-good-subarray/description/
*/
struct Solution {}

/// 類似 1856
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut ans: i64 = 0;
        let mut left_index: Vec<usize> = vec![0; n];
        let mut right_index: Vec<usize> = vec![n - 1; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..n {
            while let Some(&top) = stack.last() {
                if nums[top] >= nums[i] {
                    right_index[top] = i - 1;
                    stack.pop();
                    if right_index[top] >= k as usize && left_index[top] <= k as usize {
                        ans = ans.max(
                            (right_index[top] + 1 - left_index[top]) as i64 * nums[top] as i64,
                        );
                    }
                } else {
                    left_index[i] = top + 1;
                    break;
                }
            }
            stack.push(i);
        }

        for i in stack {
            if right_index[i] >= k as usize && left_index[i] <= k as usize {
                ans = ans.max((right_index[i] + 1 - left_index[i]) as i64 * nums[i] as i64);
            }
        }
        return ans as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::maximum_score(vec![1, 4, 3, 7, 4, 5], 3), 15);
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

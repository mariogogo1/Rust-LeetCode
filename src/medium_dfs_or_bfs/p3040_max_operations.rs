/**
3040. 相同分数的最大操作数目 II

给你一个整数数组 nums ，如果 nums 至少 包含 2 个元素，你可以执行以下操作中的 任意 一个：

选择 nums 中最前面两个元素并且删除它们。
选择 nums 中最后两个元素并且删除它们。
选择 nums 中第一个和最后一个元素并且删除它们。
一次操作的 分数 是被删除元素的和。

在确保 所有操作分数相同 的前提下，请你求出 最多 能进行多少次操作。

请你返回按照上述要求 最多 可以进行的操作次数。

https://leetcode.cn/problems/maximum-number-of-operations-with-the-same-score-ii/description/
*/
pub struct Solution;
impl Solution {
    fn dfs(start: usize, end: usize, nums: &Vec<i32>, dp: &mut Vec<Vec<i32>>, target: i32) -> i32 {
        if start >= end {
            return 0;
        }
        if dp[start][end] != -1 {
            return dp[start][end];
        }

        let mut res = 0;
        if nums[start] + nums[start + 1] == target {
            res = res.max(1 + Self::dfs(start + 2, end, nums, dp, target));
        }
        if nums[start] + nums[end] == target {
            if end > 0 {
                res = res.max(1 + Self::dfs(start + 1, end - 1, nums, dp, target));
            }
        }
        if nums[end] + nums[end - 1] == target {
            if end > 1 {
                res = res.max(1 + Self::dfs(start, end - 2, nums, dp, target));
            }
        }
        dp[start][end] = res;
        res
    }

    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![-1; n]; n];

        let mut ans = 0;
        ans = ans.max(Self::dfs(0, n - 1, &nums, &mut dp, nums[0] + nums[1]));
        dp = vec![vec![-1; n]; n];
        ans = ans.max(Self::dfs(0, n - 1, &nums, &mut dp, nums[0] + nums[n - 1]));
        dp = vec![vec![-1; n]; n];
        ans = ans.max(Self::dfs(
            0,
            n - 1,
            &nums,
            &mut dp,
            nums[n - 1] + nums[n - 2],
        ));

        ans
    }
}

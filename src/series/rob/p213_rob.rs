/**

213. 打家劫舍 II

你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都 围成一圈 ，这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警 。

给定一个代表每个房屋存放金额的非负整数数组，计算你 在不触动警报装置的情况下 ，今晚能够偷窃到的最高金额。

https://leetcode.cn/problems/house-robber-ii/description/
*/
pub struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut dp = vec![0; nums.len()];
        // 跳過LAST
        dp[1] = nums[0];
        for i in 2..nums.len() {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i - 1]);
        }
        let mut ans = dp[nums.len() - 1];

        // 跳過first
        dp[1] = nums[1];
        for i in 2..nums.len() {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }
        ans.max(dp[nums.len() - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::rob(vec![0]), 0);
    }
}

/**
494. 目标和

给你一个非负整数数组 nums 和一个整数 target 。

向数组中的每个整数前添加 '+' 或 '-' ，然后串联起所有整数，可以构造一个 表达式 ：

例如，nums = [2, 1] ，可以在 2 之前添加 '+' ，在 1 之前添加 '-' ，然后串联起来得到表达式 "+2-1" 。
返回可以通过上述方法构造的、运算结果等于 target 的不同 表达式 的数目。


https://leetcode.cn/problems/target-sum/description/
*/
struct Solution {}
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; 2001]; nums.len() + 1];
        dp[0][1000] = 1;

        for i in 0..n {
            for j in 0..2001 {
                if dp[i][j] >= 1 {
                    dp[i + 1][j - nums[i] as usize] += dp[i][j];
                    dp[i + 1][j + nums[i] as usize] += dp[i][j];
                }
            }
        }

        dp[n][(target + 1000) as usize]
    }
}

/**
377. 组合总和 Ⅳ
给你一个由 不同 整数组成的数组 nums ，和一个目标整数 target 。请你从 nums 中找出并返回总和为 target 的元素组合的个数。

题目数据保证答案符合 32 位整数范围。

https://leetcode.cn/problems/combination-sum-iv/description/
*/
pub struct Solution;
/// DFS會超時 用動態規劃，而且題目沒有要完整的答案細節，只要總數所以可以不用從DFS考慮
/// 如果一次做同題組練習可能想法會被限制住!
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 0..=target as usize {
            for j in 0..nums.len() {
                if nums[j] <= i as i32 {
                    dp[i] += dp[i - nums[j] as usize];
                }
            }
        }
        dp[target as usize]
    }
}

/**
518. 零钱兑换 II

给你一个整数数组 coins 表示不同面额的硬币，另给一个整数 amount 表示总金额。

请你计算并返回可以凑成总金额的硬币组合数。如果任何硬币组合都无法凑出总金额，返回 0 。

假设每一种面额的硬币有无限个。

题目数据保证结果符合 32 位带符号整数。

https://leetcode.cn/problems/coin-change-ii/description/
*/
/// 這題很關鍵，幾乎是遞增子序列的基礎題目，俄羅斯套娃信封這種經典題目就是從這裡延伸的
pub struct Solution;
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;

        for coin in coins {
            for i in 0..=amount {
                if i >= coin as usize {
                    dp[i] += dp[i - coin as usize];
                }
            }
        }
        dp[amount]
    }
}

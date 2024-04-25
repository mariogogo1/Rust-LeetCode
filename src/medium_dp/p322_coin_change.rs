/**
322. 零钱兑换

给你一个整数数组 coins ，表示不同面额的硬币；以及一个整数 amount ，表示总金额。

计算并返回可以凑成总金额所需的 最少的硬币个数 。如果没有任何一种硬币组合能组成总金额，返回 -1 。

你可以认为每种硬币的数量是无限的。

https://leetcode.cn/problems/coin-change/description/
*/

pub struct Solution;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 0;

        for i in 1..=amount {
            let mut min_count = i32::MAX;
            for coin in &coins {
                let x = *coin as usize;

                if i >= x && dp[i - x] >= 0 {
                    min_count = min_count.min(dp[i - x]);
                }
            }
            if min_count == i32::MAX {
                dp[i] = -1;
            } else {
                dp[i] = min_count + 1;
            }
        }

        dp[amount]
    }
}

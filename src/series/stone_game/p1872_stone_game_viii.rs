/**
1872. 石子游戏 VIII

Alice 和 Bob 玩一个游戏，两人轮流操作， Alice 先手 。

总共有 n 个石子排成一行。轮到某个玩家的回合时，如果石子的数目 大于 1 ，他将执行以下操作：

选择一个整数 x > 1 ，并且 移除 最左边的 x 个石子。
将 移除 的石子价值之 和 累加到该玩家的分数中。
将一个 新的石子 放在最左边，且新石子的值为被移除石子值之和。
当只剩下 一个 石子时，游戏结束。

Alice 和 Bob 的 分数之差 为 (Alice 的分数 - Bob 的分数) 。 Alice 的目标是 最大化 分数差，Bob 的目标是 最小化 分数差。

给你一个长度为 n 的整数数组 stones ，其中 stones[i] 是 从左边起 第 i 个石子的价值。请你返回在双方都采用 最优 策略的情况下，Alice 和 Bob 的 分数之差 。

https://leetcode.cn/problems/stone-game-viii/description/
*/

// 倒敘思考!!
impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut dp = vec![0; n];
        let mut prefix_sum = vec![stones[0]; n];

        for i in 1..stones.len() {
            prefix_sum[i] = prefix_sum[i - 1] + stones[i];
        }

        dp[n - 1] = prefix_sum[n - 1];
        for i in (0..(stones.len() - 1)).rev() {
            dp[i] = dp[i + 1].max(prefix_sum[i] - dp[i + 1]);
        }
        dp[1]
    }
}

/**
1510. 石子游戏 IV

Alice 和 Bob 两个人轮流玩一个游戏，Alice 先手。

一开始，有 n 个石子堆在一起。每个人轮流操作，正在操作的玩家可以从石子堆里拿走 任意 非零 平方数 个石子。

如果石子堆里没有石子了，则无法操作的玩家输掉游戏。

给你正整数 n ，且已知两个人都采取最优策略。如果 Alice 会赢得比赛，那么返回 True ，否则返回 False 。

https://leetcode.cn/problems/stone-game-iv/description/
*/
/// false(先手必輸) 的往前跳平方距離一定是true(先手必勝)
impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let mut w_l = vec![false; n + 1];

        for i in 0..n {
            if !w_l[i] {
                for j in 1..=(n / 2 + 1) {
                    if j * j + i > n {
                        break;
                    } else {
                        w_l[j * j + i] = true;
                    }
                }
            }
        }
        w_l[n]
    }
}

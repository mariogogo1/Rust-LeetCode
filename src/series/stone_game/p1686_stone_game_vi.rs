/**
1686. 石子游戏 VI

Alice 和 Bob 轮流玩一个游戏，Alice 先手。

一堆石子里总共有 n 个石子，轮到某个玩家时，他可以 移出 一个石子并得到这个石子的价值。Alice 和 Bob 对石子价值有 不一样的的评判标准 。双方都知道对方的评判标准。

给你两个长度为 n 的整数数组 aliceValues 和 bobValues 。aliceValues[i] 和 bobValues[i] 分别表示 Alice 和 Bob 认为第 i 个石子的价值。

所有石子都被取完后，得分较高的人为胜者。如果两个玩家得分相同，那么为平局。两位玩家都会采用 最优策略 进行游戏。

请你推断游戏的结果，用如下的方式表示：
如果 Alice 赢，返回 1 。
如果 Bob 赢，返回 -1 。
如果游戏平局，返回 0 。

https://leetcode.cn/problems/stone-game-vi/description/
*/
// A_sum -B_sum  = sum of (A[i]) - sum of (B[j]) ,i跟j 互相 補集 i是A選出的項目
//               = sum of (A[i]) + sum of (B[i]) - sum of (B[i]) - sum of (B[j])
//               = sum of (A[i] + B[i]) - total of B
impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut vec_sum = Vec::new();
        let mut total_b = 0;
        for i in 0..alice_values.len() {
            vec_sum.push(alice_values[i] + bob_values[i]);
            total_b += bob_values[i];
        }

        vec_sum.sort_unstable();

        let mut total_anb = 0;
        for val in vec_sum.iter().rev().step_by(2) {
            total_anb += val;
        }

        if total_anb > total_b {
            return 1;
        } else if total_anb == total_b {
            return 0;
        }
        -1
    }
}

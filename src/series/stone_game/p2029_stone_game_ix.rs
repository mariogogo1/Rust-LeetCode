/**
2029. 石子游戏 IX

Alice 和 Bob 再次设计了一款新的石子游戏。现有一行 n 个石子，每个石子都有一个关联的数字表示它的价值。给你一个整数数组 stones ，其中 stones[i] 是第 i 个石子的价值。

Alice 和 Bob 轮流进行自己的回合，Alice 先手。每一回合，玩家需要从 stones 中移除任一石子。

如果玩家移除石子后，导致 所有已移除石子 的价值 总和 可以被 3 整除，那么该玩家就 输掉游戏 。
如果不满足上一条，且移除后没有任何剩余的石子，那么 Bob 将会直接获胜（即便是在 Alice 的回合）。
假设两位玩家均采用 最佳 决策。如果 Alice 获胜，返回 true ；如果 Bob 获胜，返回 false 。

https://leetcode.cn/problems/stone-game-ix/description/
*/

// 這題很難，需要重新研讀
impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut cnt0 = 0;
        let mut cnt1 = 0;
        let mut cnt2 = 0;

        for &val in &stones {
            match val % 3 {
                0 => cnt0 += 1,
                1 => cnt1 += 1,
                _ => cnt2 += 1,
            }
        }

        if cnt0 % 2 == 0 {
            return cnt1 >= 1 && cnt2 >= 1;
        }

        cnt1 - cnt2 > 2 || cnt2 - cnt1 > 2
    }
}

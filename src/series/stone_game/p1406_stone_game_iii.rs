/**
1406. 石子游戏 III

Alice 和 Bob 继续他们的石子游戏。几堆石子 排成一行 ，每堆石子都对应一个得分，由数组 stoneValue 给出。

Alice 和 Bob 轮流取石子，Alice 总是先开始。在每个玩家的回合中，该玩家可以拿走剩下石子中的的前 1、2 或 3 堆石子 。比赛一直持续到所有石头都被拿走。

每个玩家的最终得分为他所拿到的每堆石子的对应得分之和。每个玩家的初始分数都是 0 。

比赛的目标是决出最高分，得分最高的选手将会赢得比赛，比赛也可能会出现平局。

假设 Alice 和 Bob 都采取 最优策略 。

如果 Alice 赢了就返回 "Alice" ，Bob 赢了就返回 "Bob"，分数相同返回 "Tie" 。

https://leetcode.cn/problems/stone-game-iii/description/
*/

// 紀錄先手選擇跟後手選擇的最大得分
impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut dp_first = vec![0; n + 1];
        let mut dp_last = vec![0; n + 1];
        let mut surfix_sum = 0;

        for i in (0..n).rev() {
            surfix_sum += stone_value[i];
            dp_first[i] = stone_value[i] + dp_last[i + 1];
            if i + 1 < n {
                dp_first[i] = dp_first[i].max(stone_value[i] + stone_value[i + 1] + dp_last[i + 2]);
            }
            if i + 2 < n {
                dp_first[i] = dp_first[i]
                    .max(stone_value[i] + stone_value[i + 1] + stone_value[i + 2] + dp_last[i + 3]);
            }
            dp_last[i] = surfix_sum - dp_first[i];
        }

        if dp_first[0] > dp_last[0] {
            return "Alice".to_string();
        } else if dp_first[0] == dp_last[0] {
            return "Tie".to_string();
        }
        return "Bob".to_string();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case() {}
}

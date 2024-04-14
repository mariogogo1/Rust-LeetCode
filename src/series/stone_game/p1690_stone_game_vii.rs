/**
1690. 石子游戏 VII

石子游戏中，爱丽丝和鲍勃轮流进行自己的回合，爱丽丝先开始 。

有 n 块石子排成一排。每个玩家的回合中，可以从行中 移除 最左边的石头或最右边的石头，并获得与该行中剩余石头值之 和 相等的得分。当没有石头可移除时，得分较高者获胜。

鲍勃发现他总是输掉游戏（可怜的鲍勃，他总是输），所以他决定尽力 减小得分的差值 。爱丽丝的目标是最大限度地 扩大得分的差值 。

给你一个整数数组 stones ，其中 stones[i] 表示 从左边开始 的第 i 个石头的值，如果爱丽丝和鲍勃都 发挥出最佳水平 ，请返回他们 得分的差值 。

https://leetcode.cn/problems/stone-game-vii/description/
*/
// 把BOB選的數字加總就是差值
// A選的得分 num[1]+...+num[n] B選的數字 num[n] 得分=num[1]+...+num[n-1]
// A的得分 - B的得分 = B選的數字 num[n]
// => 最終差值 = sum of B選的數字 num[i]
// 此題同 486提做法
// 可優化
impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut dp_a: Vec<Vec<i32>> = vec![vec![0; n]; n];
        let mut dp_diff: Vec<Vec<i32>> = vec![vec![0; n]; n];

        for i in 0..stones.len() {
            dp_a[i][i] = stones[i];
        }

        for i in (0..(n - 1)).rev() {
            for j in (i + 1)..n {
                if dp_a[i][j - 1] < dp_a[i + 1][j] {
                    // 往下
                    dp_a[i][j] = stones[i] + dp_diff[i + 1][j];
                    dp_diff[i][j] = dp_a[i + 1][j];
                } else {
                    // 往左
                    dp_a[i][j] = stones[j] + dp_diff[i][j - 1];
                    dp_diff[i][j] = dp_a[i][j - 1];
                }
            }
        }

        dp_diff[0][n - 1]
    }
}

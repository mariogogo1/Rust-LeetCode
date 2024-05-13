/**
741. 摘樱桃

给你一个 n x n 的网格 grid ，代表一块樱桃地，每个格子由以下三种数字的一种来表示：

0 表示这个格子是空的，所以你可以穿过它。
1 表示这个格子里装着一个樱桃，你可以摘到樱桃然后穿过它。
-1 表示这个格子里有荆棘，挡着你的路。
请你统计并返回：在遵守下列规则的情况下，能摘到的最多樱桃数：

从位置 (0, 0) 出发，最后到达 (n - 1, n - 1) ，只能向下或向右走，并且只能穿越有效的格子（即只可以穿过值为 0 或者 1 的格子）；
当到达 (n - 1, n - 1) 后，你要继续走，直到返回到 (0, 0) ，只能向上或向左走，并且只能穿越有效的格子；
当你经过一个格子且这个格子包含一个樱桃时，你将摘到樱桃并且这个格子会变成空的（值变为 0 ）；
如果在 (0, 0) 和 (n - 1, n - 1) 之间不存在一条可经过的路径，则无法摘到任何一个樱桃。

https://leetcode.cn/problems/cherry-pickup/description/
*/
pub struct Solution;
/// 關聯 1463
/// KEY 把題目想法改成兩個人同時從(0,0)走向(n-1,n-1)
impl Solution {
    pub fn cherry_pickup(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![vec![i32::MIN; n]; n]; 2 * n - 1];

        dp[0][0][0] = grid[0][0];

        for step in 1..(2 * n - 1) {
            // 學起來 這個寫法蠻難的!! 結合了 兩段
            for i_1 in (0.max(step as i32 - n as i32 + 1) as usize)..=(step.min(n - 1)) {
                if grid[i_1][step - i_1] != -1 {
                    for i_2 in i_1..=(step.min(n - 1)) {
                        if grid[i_2][step - i_2] != -1 {
                            dp[step][i_1][i_2] = dp[step][i_1][i_2].max(dp[step - 1][i_1][i_2]);
                            if i_2 > 0 {
                                dp[step][i_1][i_2] =
                                    dp[step][i_1][i_2].max(dp[step - 1][i_1][i_2 - 1]);
                            }
                            if i_1 > 0 {
                                dp[step][i_1][i_2] =
                                    dp[step][i_1][i_2].max(dp[step - 1][i_1 - 1][i_2]);
                                if i_2 > 0 {
                                    dp[step][i_1][i_2] =
                                        dp[step][i_1][i_2].max(dp[step - 1][i_1 - 1][i_2 - 1]);
                                }
                            }
                        }
                        if grid[i_1][step - i_1] == 1 {
                            dp[step][i_1][i_2] += 1;
                        }
                        if i_1 != i_2 && grid[i_2][step - i_2] == 1 {
                            dp[step][i_1][i_2] += 1;
                        }
                    }
                }
            }
        }

        dp[2 * n - 2][n - 1][n - 1].max(0)
    }
}

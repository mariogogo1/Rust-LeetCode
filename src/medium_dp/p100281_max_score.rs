/**
100281. 矩阵中的最大得分
给你一个由 正整数 组成、大小为 m x n 的矩阵 grid。你可以从矩阵中的任一单元格移动到另一个位于正下方或正右侧的任意单元格（不必相邻）。从值为 c1 的单元格移动到值为 c2 的单元格的得分为 c2 - c1 。

你可以从 任一 单元格开始，并且必须至少移动一次。

返回你能得到的 最大 总得分。

https://leetcode.cn/problems/maximum-difference-score-in-a-grid/description/
*/
impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut min_dp = vec![vec![0; m]; n];
        min_dp[0][0] = grid[0][0];
        let mut ans = i32::MIN;

        for i in 0..n {
            for j in 0..m {
                if i == 0 {
                    if j > 0 {
                        ans = ans.max(grid[i][j] - min_dp[i][j - 1]);
                        if grid[i][j] >= min_dp[i][j - 1] {
                            min_dp[i][j] = min_dp[i][j - 1];
                        } else {
                            min_dp[i][j] = grid[i][j];
                        }
                    }
                } else {
                    ans = ans.max(grid[i][j] - min_dp[i - 1][j]);

                    if j == 0 {
                        if grid[i][j] >= min_dp[i - 1][j] {
                            min_dp[i][j] = min_dp[i - 1][j];
                        } else {
                            min_dp[i][j] = grid[i][j];
                        }
                    } else {
                        ans = ans.max(grid[i][j] - min_dp[i][j - 1]);

                        if grid[i][j] >= min_dp[i][j - 1] {
                            min_dp[i][j] = min_dp[i][j - 1];
                        } else {
                            min_dp[i][j] = grid[i][j];
                        }
                        if min_dp[i][j] >= min_dp[i - 1][j] {
                            min_dp[i][j] = min_dp[i - 1][j];
                        }
                    }
                }
            }
        }
        ans
    }
}

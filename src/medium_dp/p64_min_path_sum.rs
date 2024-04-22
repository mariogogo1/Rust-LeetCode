/**
64. 最小路径和
给定一个包含非负整数的 m x n 网格 grid ，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。

说明：每次只能向下或者向右移动一步。

https://leetcode.cn/problems/minimum-path-sum/description/
*/
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![i32::MAX; m + 1]; n + 1];
        dp[1][0] = 0;
        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i - 1][j - 1];
            }
        }
        dp[n][m]
    }
}

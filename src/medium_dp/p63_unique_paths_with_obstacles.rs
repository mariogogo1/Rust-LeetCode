/**
63. 不同路径 II
一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish”）。

现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？

网格中的障碍物和空位置分别用 1 和 0 来表示。

https://leetcode.cn/problems/unique-paths-ii/description/
*/
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n = obstacle_grid.len();
        let m = obstacle_grid[0].len();
        let mut dp = vec![vec![0; m + 1]; n + 1];
        dp[1][1] = 1 * (1 - obstacle_grid[0][0]);
        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] += dp[i - 1][j] + dp[i][j - 1];
                dp[i][j] *= 1 - obstacle_grid[i - 1][j - 1];
            }
        }
        dp[n][m]
    }
}

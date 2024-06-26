/**
62. 不同路径
一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish” ）。

问总共有多少条不同的路径？

https://leetcode.cn/problems/unique-paths/description/
*/
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        dp[1][1] = 1;
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] += dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[m][n]
    }
}

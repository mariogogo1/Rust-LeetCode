/**

2088. 统计农场中肥沃金字塔的数目

有一个 矩形网格 状的农场，划分为 m 行 n 列的单元格。每个格子要么是 肥沃的 （用 1 表示），要么是 贫瘠 的（用 0 表示）。网格图以外的所有与格子都视为贫瘠的。

农场中的 金字塔 区域定义如下：

区域内格子数目 大于 1 且所有格子都是 肥沃的 。
金字塔 顶端 是这个金字塔 最上方 的格子。金字塔的高度是它所覆盖的行数。令 (r, c) 为金字塔的顶端且高度为 h ，那么金字塔区域内包含的任一格子 (i, j) 需满足 r <= i <= r + h - 1 且 c - (i - r) <= j <= c + (i - r) 。
一个 倒金字塔 类似定义如下：

区域内格子数目 大于 1 且所有格子都是 肥沃的 。
倒金字塔的 顶端 是这个倒金字塔 最下方 的格子。倒金字塔的高度是它所覆盖的行数。令 (r, c) 为金字塔的顶端且高度为 h ，那么金字塔区域内包含的任一格子 (i, j) 需满足 r - h + 1 <= i <= r 且 c - (r - i) <= j <= c + (r - i) 。

给你一个下标从 0 开始且大小为 m x n 的二进制矩阵 grid ，它表示农场，请你返回 grid 中金字塔和倒金字塔的 总数目 。


https://leetcode.cn/problems/count-fertile-pyramids-in-a-land/description/
*/
pub struct Solution;

impl Solution {
    pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; m]; n];
        let mut ans: i32 = 0;
        for j in 0..m {
            dp[0][j] = grid[0][j];
        }

        for i in 1..n {
            dp[i][0] = grid[i][0];
            for j in 1..m {
                if grid[i][j] == 1 {
                    dp[i][j] = dp[i][j - 1] + 1;
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1] + 2);
                }
                if dp[i][j] > 1 {
                    ans += (dp[i][j] - 1) / 2;
                }
            }
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![0; m]; n]; // reverse

        for j in 0..m {
            dp[0][j] = grid[n - 1][m - j - 1];
        }

        for i in 1..n {
            dp[i][0] = grid[n - i - 1][m - 1];
            for j in 1..m {
                if grid[n - i - 1][m - j - 1] == 1 {
                    dp[i][j] = dp[i][j - 1] + 1;
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1] + 2);
                }
                if dp[i][j] > 1 {
                    ans += (dp[i][j] - 1) / 2;
                }
            }
        }

        //println!("{:?}", dp);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::count_pyramids(vec![
                vec![1, 1, 1, 1, 0],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![0, 1, 0, 0, 1]
            ]),
            13
        );
        assert_eq!(
            Solution::count_pyramids(vec![vec![1, 1, 1], vec![1, 1, 1]]),
            2
        );

        assert_eq!(
            Solution::count_pyramids(vec![vec![0, 1, 1, 0], vec![1, 1, 1, 1]]),
            2
        );
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

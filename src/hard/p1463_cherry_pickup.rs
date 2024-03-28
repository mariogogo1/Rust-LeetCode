/**
1463. 摘樱桃 II

给你一个 rows x cols 的矩阵 grid 来表示一块樱桃地。 grid 中每个格子的数字表示你能获得的樱桃数目。

你有两个机器人帮你收集樱桃，机器人 1 从左上角格子 (0,0) 出发，机器人 2 从右上角格子 (0, cols-1) 出发。

请你按照如下规则，返回两个机器人能收集的最多樱桃数目：

从格子 (i,j) 出发，机器人可以移动到格子 (i+1, j-1)，(i+1, j) 或者 (i+1, j+1) 。
当一个机器人经过某个格子时，它会把该格子内所有的樱桃都摘走，然后这个位置会变成空格子，即没有樱桃的格子。
当两个机器人同时到达同一个格子时，它们中只有一个可以摘到樱桃。
两个机器人在任意时刻都不能移动到 grid 外面。
两个机器人最后都要到达 grid 最底下一行。

提示：
rows == grid.length
cols == grid[i].length
2 <= rows, cols <= 70
0 <= grid[i][j] <= 100

https://leetcode.cn/problems/cherry-pickup-ii/description/
*/
pub struct Solution;

/// 轉移矩陣 f(j1,j2,i) = max(f(j1,j2,i-1))+grid[i][j1]+grid[i][j2];
/// j1,j2 分別為機器人所在的COLUMN，f 兩個機器人摘總櫻桃的最大數量，i 所在row，
/// max(f(j1,j2,i-1))，每次最多遍歷3*3個位置。
/// 開設一個3維的轉移矩陣；若想節省內存可開設2維矩陣，紀錄兩個機器人的行下標。
///
/// n = grid.len();
/// m = grid[0].len();
/// 時間複雜度 O(n*m*m)  
///

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let m_i32 = m as i32;
        let mut cherry_dp = vec![vec![vec![0; m]; m]; n];
        let mut ans = 0;
        cherry_dp[0][0][m - 1] = grid[0][0] + grid[0][m - 1];
        for row in 1..n {
            for c1 in 0..m as i32 {
                for c2 in 0..m as i32 {
                    for x in -1..=1 {
                        for y in -1..=1 {
                            if c1 + x >= 0
                                && c1 + x < m_i32
                                && c1 + x < row as i32
                                && c2 + y >= 0
                                && c2 + y < m_i32
                                && c2 + y > m_i32 - 1 - row as i32
                            {
                                cherry_dp[row][c1 as usize][c2 as usize] = cherry_dp[row]
                                    [c1 as usize][c2 as usize]
                                    .max(cherry_dp[row - 1][(c1 + x) as usize][(c2 + y) as usize]);
                            }
                        }
                    }
                    if c1 == c2 {
                        cherry_dp[row][c1 as usize][c2 as usize] += grid[row][c1 as usize];
                    } else {
                        cherry_dp[row][c1 as usize][c2 as usize] +=
                            grid[row][c1 as usize] + grid[row][c2 as usize];
                    }

                    if row == n - 1 {
                        ans = ans.max(cherry_dp[n - 1][c1 as usize][c2 as usize]);
                    }
                }
            }
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::cherry_pickup(vec![
                vec![1, 2, 3, 4, 5],
                vec![1, 2, 3, 4, 5],
                vec![1, 2, 3, 4, 5]
            ]),
            21
        );
    }
    #[test]
    fn test_case() {
        assert_eq!(
            Solution::cherry_pickup(vec![
                vec![3, 1, 1],
                vec![2, 5, 1],
                vec![1, 5, 5],
                vec![2, 1, 1],
            ]),
            24
        );
        assert_eq!(
            Solution::cherry_pickup(vec![
                vec![0, 8, 7, 10, 9, 10, 0, 9, 6],
                vec![8, 7, 10, 8, 7, 4, 9, 6, 10],
                vec![8, 1, 1, 5, 1, 5, 5, 1, 2],
                vec![9, 4, 10, 8, 8, 1, 9, 5, 0],
                vec![4, 3, 6, 10, 9, 2, 4, 8, 10],
                vec![7, 3, 2, 8, 3, 3, 5, 9, 8],
                vec![1, 2, 6, 5, 6, 2, 0, 10, 0],
            ]),
            96
        );
    }
}

/**
100299. 判断矩阵是否满足条件

给你一个大小为 m x n 的二维矩阵 grid 。你需要判断每一个格子 grid[i][j] 是否满足：

如果它下面的格子存在，那么它需要等于它下面的格子，也就是 grid[i][j] == grid[i + 1][j] 。
如果它右边的格子存在，那么它需要不等于它右边的格子，也就是 grid[i][j] != grid[i][j + 1] 。
如果 所有 格子都满足以上条件，那么返回 true ，否则返回 false 。

https://leetcode.cn/contest/biweekly-contest-130/problems/check-if-grid-satisfies-conditions/
*/

pub struct Solution;
impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        let m = grid[0].len();

        for j in 1..m {
            if grid[0][j] == grid[0][j - 1] {
                return false;
            }
        }

        for i in 1..n {
            for j in 0..m {
                if grid[i][j] != grid[i - 1][j] {
                    return false;
                }
                if j > 0 {
                    if grid[i][j] == grid[i][j - 1] {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

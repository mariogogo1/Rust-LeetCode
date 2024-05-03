/**
3128. 直角三角形

给你一个二维 boolean 矩阵 grid 。

请你返回使用 grid 中的 3 个元素可以构建的 直角三角形 数目，且满足 3 个元素值 都 为 1 。

注意：

如果 grid 中 3 个元素满足：一个元素与另一个元素在 同一行，同时与第三个元素在 同一列 ，那么这 3 个元素称为一个 直角三角形 。这 3 个元素互相之间不需要相邻。


https://leetcode.cn/problems/right-triangles/description/
*/

pub struct Solution;

impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        let m = grid[0].len();
        let mut row_count: Vec<i64> = vec![0; n];
        let mut col_count: Vec<i64> = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    row_count[i] += 1;
                    col_count[j] += 1;
                }
            }
        }

        let mut ans: i64 = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    ans += (row_count[i] - 1) * (col_count[j] - 1);
                }
            }
        }

        ans
    }
}

/**
1034. 边界着色

给你一个大小为 m x n 的整数矩阵 grid ，表示一个网格。另给你三个整数 row、col 和 color 。网格中的每个值表示该位置处的网格块的颜色。

如果两个方块在任意 4 个方向上相邻，则称它们 相邻 。

如果两个方块具有相同的颜色且相邻，它们则属于同一个 连通分量 。

连通分量的边界 是指连通分量中满足下述条件之一的所有网格块：

在上、下、左、右任意一个方向上与不属于同一连通分量的网格块相邻
在网格的边界上（第一行/列或最后一行/列）
请你使用指定颜色 color 为所有包含网格块 grid[row][col] 的 连通分量的边界 进行着色。

并返回最终的网格 grid 。

https://leetcode.cn/problems/coloring-a-border/description/
*/
pub struct Solution;

struct Directions {
    steps: Vec<(i32, i32)>,
}

impl Directions {
    pub fn new() -> Self {
        return Directions {
            steps: vec![(1, 0), (0, 1), (-1, 0), (0, -1)],
        };
    }
}

impl Solution {
    pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let dirs = Directions::new();
        let n = grid.len();
        let m = grid[0].len();
        let target = grid[row as usize][col as usize];

        Self::dfs(&mut grid, row as usize, col as usize, &dirs, target);

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == -1 {
                    grid[i][j] = color;
                }
                if grid[i][j] == -2 {
                    grid[i][j] = target;
                }
            }
        }

        grid
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, dirs: &Directions, target: i32) {
        if grid[x][y] == target {
            for dir in &dirs.steps {
                let n_x = x as i32 + dir.0;
                let n_y = y as i32 + dir.1;

                if n_x >= 0 && n_x < grid.len() as i32 && n_y >= 0 && n_y < grid[0].len() as i32 {
                    if grid[n_x as usize][n_y as usize] > 0
                        && grid[n_x as usize][n_y as usize] != target
                    {
                        grid[x][y] = -1;
                    }
                } else {
                    grid[x][y] = -1;
                }
            }
            if grid[x][y] != -1 {
                grid[x][y] = -2;
            }

            for dir in &dirs.steps {
                let n_x = x as i32 + dir.0;
                let n_y = y as i32 + dir.1;

                if n_x >= 0 && n_x < grid.len() as i32 && n_y >= 0 && n_y < grid[0].len() as i32 {
                    Self::dfs(grid, n_x as usize, n_y as usize, dirs, target);
                }
            }
        } else {
            return;
        }
    }
}

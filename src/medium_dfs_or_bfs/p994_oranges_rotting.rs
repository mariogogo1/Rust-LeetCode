/**
994. 腐烂的橘子

在给定的 m x n 网格 grid 中，每个单元格可以有以下三个值之一：

值 0 代表空单元格；
值 1 代表新鲜橘子；
值 2 代表腐烂的橘子。
每分钟，腐烂的橘子 周围 4 个方向上相邻 的新鲜橘子都会腐烂。

返回 直到单元格中没有新鲜橘子为止所必须经过的最小分钟数。如果不可能，返回 -1 。

https://leetcode.cn/problems/rotting-oranges/description/
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
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut fresh_counts = 0;
        let mut min = -1;
        let n = grid.len();
        let m = grid[0].len();
        let mut rotting = vec![];
        let dirs = Directions::new();

        for (rowkey, row) in grid.iter().enumerate() {
            for (colkey, &orange) in row.iter().enumerate() {
                if orange == 1 {
                    fresh_counts += 1;
                }
                if orange == 2 {
                    rotting.push(rowkey * m + colkey);
                }
            }
        }

        if fresh_counts == 0 {
            return 0;
        }

        while !rotting.is_empty() {
            min += 1;
            let mut extend = vec![];
            for &value in &rotting {
                let row = value / m;
                let col = value % m;
                for (dx, dy) in &dirs.steps {
                    let new_row = (row as i32 + dx) as usize;
                    let new_col = (col as i32 + dy) as usize;
                    if new_row < n && new_col < m && grid[new_row][new_col] == 1 {
                        fresh_counts -= 1;
                        grid[new_row][new_col] = 2;
                        extend.push(new_row * m + new_col);
                    }
                }
            }
            rotting = extend;
        }

        if fresh_counts > 0 {
            return -1;
        }

        min
    }
}

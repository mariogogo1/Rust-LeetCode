/**
200. 岛屿数量

给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。

岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。

此外，你可以假设该网格的四条边均被水包围。

https://leetcode.cn/problems/number-of-islands/description/
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
    fn dfs(grid: &mut Vec<Vec<char>>, x: usize, y: usize, dirs: &Directions) {
        grid[x][y] = '2';
        for dir in &dirs.steps {
            let x = x as i32 + dir.0;
            let y = y as i32 + dir.1;

            if x >= 0
                && x < grid.len() as i32
                && y >= 0
                && y < grid[0].len() as i32
                && grid[x as usize][y as usize] == '1'
            {
                Self::dfs(grid, x as usize, y as usize, dirs);
            }
        }
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count: i32 = 0;
        let dirs = Directions::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    count += 1;
                    Self::dfs(&mut grid, i, j, &dirs);
                }
            }
        }
        return count;
    }
}

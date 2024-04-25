/**
463. 岛屿的周长

给定一个 row x col 的二维网格地图 grid ，其中：grid[i][j] = 1 表示陆地， grid[i][j] = 0 表示水域。

网格中的格子 水平和垂直 方向相连（对角线方向不相连）。整个网格被水完全包围，但其中恰好有一个岛屿（或者说，一个或多个表示陆地的格子相连组成的岛屿）。

岛屿中没有“湖”（“湖” 指水域在岛屿内部且不和岛屿周围的水相连）。格子是边长为 1 的正方形。网格为长方形，且宽度和高度均不超过 100 。计算这个岛屿的周长。

https://leetcode.cn/problems/island-perimeter/description/
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
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter: i32 = 0;
        let dirs = Directions::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    perimeter += 4;

                    for dir in &dirs.steps {
                        let x = i as i32 + dir.0;
                        let y = j as i32 + dir.1;

                        if x >= 0
                            && x < grid.len() as i32
                            && y >= 0
                            && y < grid[0].len() as i32
                            && grid[x as usize][y as usize] == 1
                        {
                            perimeter -= 1;
                        }
                    }
                }
            }
        }
        return perimeter;
    }
}

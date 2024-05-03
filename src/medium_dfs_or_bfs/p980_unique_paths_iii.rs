/**
980. 不同路径 III

在二维网格 grid 上，有 4 种类型的方格：

1 表示起始方格。且只有一个起始方格。
2 表示结束方格，且只有一个结束方格。
0 表示我们可以走过的空方格。
-1 表示我们无法跨越的障碍。
返回在四个方向（上、下、左、右）上行走时，从起始方格到结束方格的不同路径的数目。

每一个无障碍方格都要通过一次，但是一条路径中不能重复通过同一个方格。

https://leetcode.cn/problems/unique-paths-iii/description/
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
    fn dfs(
        grid: &mut Vec<Vec<i32>>,
        x: usize,
        y: usize,
        dirs: &Directions,
        count: &mut i32,
        ans: &mut i32,
    ) {
        // println!("{}",count);
        if grid[x][y] == 2 && *count == 0 {
            *ans += 1;
            return;
        }
        let s = grid[x][y];
        grid[x][y] = 3;
        *count -= 1;
        for dir in &dirs.steps {
            let x = (x as i32 + dir.0) as usize;
            let y = (y as i32 + dir.1) as usize;

            if x >= 0 && x < grid.len() && y >= 0 && y < grid[0].len() {
                if grid[x][y] == 0 || grid[x][y] == 2 {
                    Self::dfs(grid, x, y, dirs, count, ans);
                }
            }
        }
        *count += 1;

        grid[x][y] = s;
    }
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut count: i32 = 1;
        let mut ans: i32 = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    count += 1;
                }
            }
        }

        let dirs = Directions::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    Self::dfs(&mut grid, i, j, &dirs, &mut count, &mut ans);
                }
            }
        }

        return ans;
    }
}

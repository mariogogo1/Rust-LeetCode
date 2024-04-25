/**
695. 岛屿的最大面积

给你一个大小为 m x n 的二进制矩阵 grid 。

岛屿 是由一些相邻的 1 (代表土地) 构成的组合，这里的「相邻」要求两个 1 必须在 水平或者竖直的四个方向上 相邻。你可以假设 grid 的四个边缘都被 0（代表水）包围着。

岛屿的面积是岛上值为 1 的单元格的数目。

计算并返回 grid 中最大的岛屿面积。如果没有岛屿，则返回面积为 0 。

https://leetcode.cn/problems/max-area-of-island/description/
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
    fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, area: &mut i32, dirs: &Directions) {
        *area += 1;
        grid[x][y] = 2;
        for dir in &dirs.steps {
            let x = x as i32 + dir.0;
            let y = y as i32 + dir.1;

            if x >= 0
                && x < grid.len() as i32
                && y >= 0
                && y < grid[0].len() as i32
                && grid[x as usize][y as usize] == 1
            {
                Self::dfs(grid, x as usize, y as usize, area, dirs);
            }
        }
    }

    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut area: i32 = 0;
        let dirs = Directions::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    let mut new_area: i32 = 0;
                    Self::dfs(&mut grid, i, j, &mut new_area, &dirs);
                    area = area.max(new_area)
                }
            }
        }
        return area;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case() {}
}

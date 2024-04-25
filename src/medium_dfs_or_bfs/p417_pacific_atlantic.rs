/**
417. 太平洋大西洋水流问题

有一个 m × n 的矩形岛屿，与 太平洋 和 大西洋 相邻。 “太平洋” 处于大陆的左边界和上边界，而 “大西洋” 处于大陆的右边界和下边界。

这个岛被分割成一个由若干方形单元格组成的网格。给定一个 m x n 的整数矩阵 heights ， heights[r][c] 表示坐标 (r, c) 上单元格 高于海平面的高度 。

岛上雨水较多，如果相邻单元格的高度 小于或等于 当前单元格的高度，雨水可以直接向北、南、东、西流向相邻单元格。水可以从海洋附近的任何单元格流入海洋。

返回网格坐标 result 的 2D 列表 ，其中 result[i] = [ri, ci] 表示雨水从单元格 (ri, ci) 流动 既可流向太平洋也可流向大西洋 。

https://leetcode.cn/problems/pacific-atlantic-water-flow/description/
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
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let dirs = Directions::new();
        let n = heights.len();
        let m = heights[0].len();

        let mut water_map = vec![vec![0; m]; n];

        for i in 0..n {
            if water_map[i][0] != 1 && water_map[i][0] != 3 {
                Self::dfs(&mut water_map, &heights, i, 0, &dirs, 1);
            }
        }
        for j in 0..m {
            if water_map[0][j] != 1 && water_map[0][j] != 3 {
                Self::dfs(&mut water_map, &heights, 0, j, &dirs, 1);
            }
        }

        for i in 0..n {
            if water_map[i][m - 1] != 2 && water_map[i][m - 1] != 3 {
                Self::dfs(&mut water_map, &heights, i, m - 1, &dirs, 2);
            }
        }
        for j in 0..m {
            if water_map[n - 1][j] != 2 && water_map[n - 1][j] != 3 {
                Self::dfs(&mut water_map, &heights, n - 1, j, &dirs, 2);
            }
        }

        let mut ans = Vec::new();

        for i in 0..n {
            for j in 0..m {
                if water_map[i][j] == 3 {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }
        ans
    }

    fn dfs(
        water_map: &mut Vec<Vec<i32>>,
        heights: &Vec<Vec<i32>>,
        x: usize,
        y: usize,
        dirs: &Directions,
        water_type: i32,
    ) {
        water_map[x][y] += water_type;
        for dir in &dirs.steps {
            let n_x = x as i32 + dir.0;
            let n_y = y as i32 + dir.1;

            if n_x >= 0
                && n_x < water_map.len() as i32
                && n_y >= 0
                && n_y < water_map[0].len() as i32
            {
                let n_x = n_x as usize;
                let n_y = n_y as usize;

                if heights[n_x][n_y] >= heights[x][y] {
                    if water_map[n_x][n_y] != water_type && water_map[n_x][n_y] != 3 {
                        Self::dfs(water_map, heights, n_x, n_y, &dirs, water_type);
                    }
                }
            }
        }
    }
}

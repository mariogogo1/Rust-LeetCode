/**
329. 矩阵中的最长递增路径

给定一个 m x n 整数矩阵 matrix ，找出其中 最长递增路径 的长度。

对于每个单元格，你可以往上，下，左，右四个方向移动。 你 不能 在 对角线 方向上移动或移动到 边界外（即不允许环绕）。

https://leetcode.cn/problems/longest-increasing-path-in-a-matrix/description/
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
/// DP跟DFS混合問題 類似1340跳躍遊戲V
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let dirs = Directions::new();

        let mut dp: Vec<Vec<i32>> = vec![vec![0; m]; n];

        let mut ans: i32 = 0;

        for i in 0..n {
            for j in 0..m {
                ans = ans.max(Self::dfs(&mut dp, &matrix, &dirs, i, j));
            }
        }

        ans
    }

    fn dfs(
        dp: &mut Vec<Vec<i32>>,
        matrix: &Vec<Vec<i32>>,
        dirs: &Directions,
        row: usize,
        col: usize,
    ) -> i32 {
        if dp[row][col] != 0 {
            return dp[row][col];
        }
        let mut ans = 0;

        for dir in &dirs.steps {
            let x = row as i32 + dir.0;
            let y = col as i32 + dir.1;

            if x >= 0
                && x < dp.len() as i32
                && y >= 0
                && y < dp[0].len() as i32
                && matrix[x as usize][y as usize] < matrix[row][col]
            {
                ans = ans.max(Self::dfs(dp, matrix, dirs, x as usize, y as usize));
            }
        }

        dp[row][col] = ans + 1;
        dp[row][col]
    }
}

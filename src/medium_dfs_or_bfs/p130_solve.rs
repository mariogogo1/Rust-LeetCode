/**
130. 被围绕的区域

给你一个 m x n 的矩阵 board ，由若干字符 'X' 和 'O' ，找到所有被 'X' 围绕的区域，并将这些区域里所有的 'O' 用 'X' 填充。

https://leetcode.cn/problems/surrounded-regions/description/
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
        board: &mut Vec<Vec<char>>,
        x: usize,
        y: usize,
        dirs: &Directions,
        target: char,
        change: char,
    ) {
        if board[x][y] == target {
            board[x][y] = change;
            for dir in &dirs.steps {
                let x = x as i32 + dir.0;
                let y = y as i32 + dir.1;

                if x >= 0 && x < board.len() as i32 && y >= 0 && y < board[0].len() as i32 {
                    Self::dfs(board, x as usize, y as usize, dirs, target, change);
                }
            }
        } else {
            return;
        }
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        let dirs = Directions::new();
        let n = board.len();
        let m = board[0].len();

        for i in 0..n {
            Self::dfs(board, i, 0, &dirs, 'O', '-');
            Self::dfs(board, i, m - 1, &dirs, 'O', '-');
        }
        for j in 0..m {
            Self::dfs(board, 0, j, &dirs, 'O', '-');
            Self::dfs(board, n - 1, j, &dirs, 'O', '-');
        }

        for i in 0..n {
            for j in 0..m {
                Self::dfs(board, i, j, &dirs, 'O', 'X');
            }
        }

        for i in 0..n {
            for j in 0..m {
                Self::dfs(board, i, j, &dirs, '-', 'O');
            }
        }
    }
}

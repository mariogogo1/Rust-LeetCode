/**
79. 单词搜索

给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word 存在于网格中，返回 true ；否则，返回 false 。

单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。

https://leetcode.cn/problems/word-search/description/
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
        words: &[char],
    ) -> bool {
        if board[x][y] != words[0] {
            return false;
        }
        if words.len() == 1 {
            return true;
        }
        board[x][y] = '#';
        for dir in &dirs.steps {
            let n_x = x as i32 + dir.0;
            let n_y = y as i32 + dir.1;

            if n_x >= 0
                && n_x < board.len() as i32
                && n_y >= 0
                && n_y < board[0].len() as i32
                && board[n_x as usize][n_y as usize] != '#'
            {
                if Self::dfs(board, n_x as usize, n_y as usize, dirs, &words[1..]) {
                    return true;
                }
            }
        }
        board[x][y] = words[0];

        return false;
    }

    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let dirs = Directions::new();
        let words: &[char] = &word.chars().collect::<Vec<char>>();

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let ans = Self::dfs(&mut board, i, j, &dirs, words);
                if ans {
                    return true;
                }
            }
        }
        false
    }
}

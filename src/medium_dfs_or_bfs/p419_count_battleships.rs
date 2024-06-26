/**
419. 甲板上的战舰

给你一个大小为 m x n 的矩阵 board 表示甲板，其中，每个单元格可以是一艘战舰 'X' 或者是一个空位 '.' ，返回在甲板 board 上放置的 战舰 的数量。

战舰 只能水平或者垂直放置在 board 上。换句话说，战舰只能按 1 x k（1 行，k 列）或 k x 1（k 行，1 列）的形状建造，其中 k 可以是任意大小。两艘战舰之间至少有一个水平或垂直的空位分隔 （即没有相邻的战舰）。

https://leetcode.cn/problems/battleships-in-a-board/description/
*/
pub struct Solution;

// 基本作法可參考 200 島嶼數量 使用DFS
// 进阶：你可以实现一次扫描算法，并只使用 O(1) 额外空间，并且不修改 board 的值来解决这个问题吗？

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'X' {
                    let mut up = false;
                    let mut left = false;

                    if i != 0 && board[i - 1][j] == 'X' {
                        up = true;
                    }
                    if j != 0 && board[i][j - 1] == 'X' {
                        left = true;
                    }

                    if !up && !left {
                        ans += 1;
                    }
                }
            }
        }

        ans
    }
}

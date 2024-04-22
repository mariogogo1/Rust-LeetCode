/**
36. 有效的数独

请你判断一个 9 x 9 的数独是否有效。只需要 根据以下规则 ，验证已经填入的数字是否有效即可。

数字 1-9 在每一行只能出现一次。
数字 1-9 在每一列只能出现一次。
数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。（请参考示例图）


注意：

一个有效的数独（部分已被填充）不一定是可解的。
只需要根据以上规则，验证已经填入的数字是否有效即可。
空白格用 '.' 表示。

https://leetcode.cn/problems/valid-sudoku/description/
*/
pub struct Solution;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [[0; 9]; 9];
        let mut columns = [[0; 9]; 9];
        let mut subboxes = [[[0; 9]; 3]; 3];

        for i in 0..9 {
            for j in 0..9 {
                let c = board[i][j];
                if c == '.' {
                    continue;
                }
                let index = (c as usize) - ('1' as usize);
                rows[i][index] += 1;
                columns[j][index] += 1;
                subboxes[i / 3][j / 3][index] += 1;
                if rows[i][index] > 1 || columns[j][index] > 1 || subboxes[i / 3][j / 3][index] > 1
                {
                    return false;
                }
            }
        }
        true
    }
}

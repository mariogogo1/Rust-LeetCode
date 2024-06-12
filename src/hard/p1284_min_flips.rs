/**

1284. 转化为全零矩阵的最少反转次数

给你一个 m x n 的二进制矩阵 mat。每一步，你可以选择一个单元格并将它反转（反转表示 0 变 1 ，1 变 0 ）。如果存在和它相邻的单元格，那么这些相邻的单元格也会被反转。相邻的两个单元格共享同一条边。

请你返回将矩阵 mat 转化为全零矩阵的最少反转次数，如果无法转化为全零矩阵，请返回 -1 。

二进制矩阵 的每一个格子要么是 0 要么是 1 。

全零矩阵 是所有格子都为 0 的矩阵。

https://leetcode.cn/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/description/
*/
pub struct Solution;
impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut state = vec![-1; 1 << 9];
        let mut init = 0 as usize;

        for i in 0..n {
            for j in 0..m {
                init <<= 1;
                init |= mat[i][j] as usize;
            }
        }

        state[init] = 0;

        let mut depart = vec![init];

        while depart.len() > 0 {
            let mut arr = vec![];
            for &pos in depart.iter() {
                for i in 0..n {
                    for j in 0..m {
                        let mut new_pos = pos;
                        new_pos ^= 1 << i * m + j;
                        if i > 0 {
                            new_pos ^= 1 << (i - 1) * m + j;
                        }
                        if i < n - 1 {
                            new_pos ^= 1 << (i + 1) * m + j;
                        }
                        if j > 0 {
                            new_pos ^= 1 << i * m + j - 1;
                        }
                        if j < m - 1 {
                            new_pos ^= 1 << i * m + j + 1;
                        }
                        if state[new_pos] > -1 {
                            continue;
                        }
                        arr.push(new_pos);
                        state[new_pos] = state[pos] + 1;
                        if new_pos == 0 {
                            return state[new_pos];
                        }
                    }
                }
            }

            depart = arr.clone();
        }
        state[0]
    }
}

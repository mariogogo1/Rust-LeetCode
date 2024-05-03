/**
1329. 将矩阵按对角线排序

矩阵对角线 是一条从矩阵最上面行或者最左侧列中的某个元素开始的对角线，沿右下方向一直到矩阵末尾的元素。例如，矩阵 mat 有 6 行 3 列，从 mat[2][0] 开始的 矩阵对角线 将会经过 mat[2][0]、mat[3][1] 和 mat[4][2] 。

给你一个 m * n 的整数矩阵 mat ，请你将同一条 矩阵对角线 上的元素按升序排序后，返回排好序的矩阵。


https://leetcode.cn/problems/sort-the-matrix-diagonally/description
*/
pub struct Solution;
impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut n = mat.len() as i32;
        let mut m = mat[0].len() as i32;
        let mut ans_array: Vec<Vec<i32>> = vec![vec![0; m as usize]; n as usize];

        for x in (-n + 1)..m {
            let mut dia = Vec::new();
            for y in 0..n {
                if x + y < 0 {
                    continue;
                }
                if x + y >= m {
                    break;
                }

                dia.push(mat[y as usize][(x + y) as usize]);
            }
            dia.sort_unstable();
            let mut idx = 0 as usize;
            for y in 0..n {
                if x + y < 0 {
                    continue;
                }
                if x + y >= m {
                    break;
                }
                ans_array[y as usize][(x + y) as usize] = dia[idx];
                idx += 1;
            }
        }
        ans_array
    }
}

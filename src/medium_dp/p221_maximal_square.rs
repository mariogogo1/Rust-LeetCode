/**
221. 最大正方形

在一个由 '0' 和 '1' 组成的二维矩阵内，找到只包含 '1' 的最大正方形，并返回其面积。

https://leetcode.cn/problems/maximal-square/
*/
/// 提供動態規劃解法 不同於 84 /85 題
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut max_v = 0;
        let mut max_edge = vec![0; m];
        let mut old_j;
        let mut new_j = i32::MAX;

        for i in 0..n {
            for j in 0..m {
                old_j = max_edge[j];
                if matrix[i][j] == '0' {
                    max_edge[j] = 0;
                    continue;
                } else {
                    if i == 0 || j == 0 {
                        max_edge[j] = 1;
                    } else {
                        max_edge[j] = 1 + new_j.min(max_edge[j]).min(max_edge[j - 1]);
                    }
                    max_v = max_v.max(max_edge[j]);
                }
                new_j = old_j;
            }
        }

        (max_v * max_v) as i32
    }
}

/**
668. 乘法表中第k小的数

几乎每一个人都用 乘法表。但是你能在乘法表中快速找到第 k 小的数字吗？

乘法表是大小为 m x n 的一个整数矩阵，其中 mat[i][j] == i * j（下标从 1 开始）。

给你三个整数 m、n 和 k，请你在大小为 m x n 的乘法表中，找出并返回第 k 小的数字。

https://leetcode.cn/problems/kth-smallest-number-in-multiplication-table/description/
*/

pub struct Solution;
impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let (mut l, mut r) = (1, m * n);

        while l < r {
            let mid = (l + r) / 2;
            let mut sum = 0;
            let (mut i, mut j) = (m, 1);

            while i >= 1 && j <= n {
                if i * j > mid {
                    i -= 1;
                } else {
                    j += 1;
                    sum += i;
                }
            }

            if sum >= k {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        r
    }
}

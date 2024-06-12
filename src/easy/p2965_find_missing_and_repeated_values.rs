/**
2965. 找出缺失和重复的数字

给你一个下标从 0 开始的二维整数矩阵 grid，大小为 n * n ，其中的值在 [1, n2] 范围内。除了 a 出现 两次，b 缺失 之外，每个整数都 恰好出现一次 。

任务是找出重复的数字a 和缺失的数字 b 。

返回一个下标从 0 开始、长度为 2 的整数数组 ans ，其中 ans[0] 等于 a ，ans[1] 等于 b 。
https://leetcode.cn/problems/find-missing-and-repeated-values/description/
*/

pub struct Solution;
impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut exist: Vec<bool> = vec![false; n * n + 1];
        let mut total = (n * n * (n * n + 1) / 2) as i32;
        let mut b = 0;

        let mut count = 1;
        for row in grid.iter() {
            for &num in row.iter() {
                if !exist[num as usize] {
                    exist[num as usize] = true;
                    b ^= num;
                }
                b ^= count;
                count += 1;
                total -= num;
            }
        }

        vec![b - total, b]
    }
}

/**
100286. 构造相同颜色的正方形

给你一个二维 3 x 3 的矩阵 grid ，每个格子都是一个字符，要么是 'B' ，要么是 'W' 。字符 'W' 表示白色，字符 'B' 表示黑色。

你的任务是改变 至多一个 格子的颜色，使得矩阵中存在一个 2 x 2 颜色完全相同的正方形。

如果可以得到一个相同颜色的 2 x 2 正方形，那么返回 true ，否则返回 false 。

https://leetcode.cn/problems/make-a-square-with-the-same-color/description/
*/

pub struct Solution;

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                let mut black = 0;
                let mut white = 0;
                for h in 0..2 {
                    for n in 0..2 {
                        if grid[i + h][j + n] == 'B' {
                            black += 1;
                        } else {
                            white += 1;
                        }
                    }
                }
                if black == white {
                    continue;
                } else {
                    return true;
                }
            }
        }
        false
    }
}

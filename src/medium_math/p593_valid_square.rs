/**
593. 有效的正方形

给定2D空间中四个点的坐标 p1, p2, p3 和 p4，如果这四个点构成一个正方形，则返回 true 。

点的坐标 pi 表示为 [xi, yi] 。 输入没有任何顺序 。

一个 有效的正方形 有四条等边和四个等角(90度角)。

https://leetcode.cn/problems/valid-square/description/
*/

pub struct Solution;
impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut points = vec![p1, p2, p3, p4];
        points.sort_unstable_by(|x, y| {
            if x[0] != y[0] {
                return x[0].cmp(&y[0]);
            }
            return x[1].cmp(&y[1]);
        });

        if points[3][0] + points[0][0] == points[2][0] + points[1][0] {
            let dai_1 = (points[3][0] - points[0][0], points[3][1] - points[0][1]);
            let dai_2 = (points[2][0] - points[1][0], points[2][1] - points[1][1]);
            if dai_1.1 == 0 && dai_2.0 == 0 {
                if dai_1.0 == dai_2.1 && dai_1.0 != 0 {
                    return true;
                }
                return false;
            }
            if dai_1.1 * dai_2.1 == -1 * dai_1.0 * dai_2.0 {
                if dai_1.0 * dai_1.0 + dai_1.1 * dai_1.1 == dai_2.0 * dai_2.0 + dai_2.1 * dai_2.1 {
                    return true;
                }
            }
        }
        return false;
    }
}

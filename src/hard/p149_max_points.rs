/**

149. 直线上最多的点数

给你一个数组 points ，其中 points[i] = [xi, yi] 表示 X-Y 平面上的一个点。求最多有多少个点在同一条直线上。

https://leetcode.cn/problems/max-points-on-a-line/description/
*/
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 2 {
            return points.len() as i32;
        }

        let mut max_points = 1;

        for (index_a, point_a) in points.iter().enumerate() {
            let mut slopes: HashMap<(i32, i32), i32> = HashMap::new();

            for (index_b, point_b) in points.iter().enumerate() {
                if index_a != index_b {
                    let dx = point_b[0] - point_a[0];
                    let dy = point_b[1] - point_a[1];
                    let gcd = Solution::gcd(dx, dy);
                    let dx = dx / gcd;
                    let dy = dy / gcd;

                    let slope = if dx < 0 { (-dx, -dy) } else { (dx, dy) };

                    *slopes.entry(slope).or_insert(0) += 1;
                }
            }

            for &count in slopes.values() {
                if max_points < count + 1 {
                    max_points = count + 1;
                }
            }
        }

        max_points
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a.abs()
        } else {
            Solution::gcd(b, a % b)
        }
    }
}

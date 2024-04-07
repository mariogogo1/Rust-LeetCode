/**
223. 矩形面积

给你 二维 平面上两个 由直线构成且边与坐标轴平行/垂直 的矩形，请你计算并返回两个矩形覆盖的总面积。

每个矩形由其 左下 顶点和 右上 顶点坐标表示：

第一个矩形由其左下顶点 (ax1, ay1) 和右上顶点 (ax2, ay2) 定义。
第二个矩形由其左下顶点 (bx1, by1) 和右上顶点 (bx2, by2) 定义。

https://leetcode.cn/problems/rectangle-area/description/
*/

pub struct Solution;

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let total_area = (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1);
        let mut overlap_height = 0;
        let mut overlap_width = 0;

        if ax1 >= bx2 || bx1 >= ax2 {
            return total_area;
        } else {
            if ax2 > bx2 {
                if ax1 > bx1 {
                    overlap_width = bx2 - ax1;
                } else {
                    overlap_width = bx2 - bx1;
                }
            } else {
                if ax1 > bx1 {
                    overlap_width = ax2 - ax1;
                } else {
                    overlap_width = ax2 - bx1;
                }
            }
        }
        if ay1 >= by2 || by1 >= ay2 {
            return total_area;
        } else {
            if ay2 > by2 {
                if ay1 > by1 {
                    overlap_height = by2 - ay1;
                } else {
                    overlap_height = by2 - by1;
                }
            } else {
                if ay1 > by1 {
                    overlap_height = ay2 - ay1;
                } else {
                    overlap_height = ay2 - by1;
                }
            }
        }
        total_area - overlap_height * overlap_width
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
        assert_eq!(Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

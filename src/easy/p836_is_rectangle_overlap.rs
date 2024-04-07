/**
836. 矩形重叠

矩形以列表 [x1, y1, x2, y2] 的形式表示，其中 (x1, y1) 为左下角的坐标，(x2, y2) 是右上角的坐标。矩形的上下边平行于 x 轴，左右边平行于 y 轴。

如果相交的面积为 正 ，则称两矩形重叠。需要明确的是，只在角或边接触的两个矩形不构成重叠。

给出两个矩形 rec1 和 rec2 。如果它们重叠，返回 true；否则，返回 false 。

https://leetcode.cn/problems/rectangle-overlap/description/
*/
/// 綑綁題目 223. 矩形面积
pub struct Solution;

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        if rec1[0] >= rec2[2] || rec2[0] >= rec1[2] {
            return false;
        }
        if rec1[1] >= rec2[3] || rec2[1] >= rec1[3] {
            return false;
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]),
            true
        );
        assert_eq!(
            Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1]),
            false
        );
        assert_eq!(
            Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![2, 2, 3, 3]),
            false
        );
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

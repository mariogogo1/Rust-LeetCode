/**
3143. 正方形中的最多点数

给你一个二维数组 points 和一个字符串 s ，其中 points[i] 表示第 i 个点的坐标，s[i] 表示第 i 个点的 标签 。

如果一个正方形的中心在 (0, 0) ，所有边都平行于坐标轴，且正方形内 不 存在标签相同的两个点，那么我们称这个正方形是 合法 的。

请你返回 合法 正方形中可以包含的 最多 点数。

注意：

如果一个点位于正方形的边上或者在边以内，则认为该点位于正方形内。
正方形的边长可以为零。

https://leetcode.cn/problems/maximum-points-inside-the-square/description/
*/

pub struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut radius_pair: Vec<(i32, char)> = Vec::new();
        let n = points.len();
        let s_vec: Vec<char> = s.chars().collect();

        for i in 0..n {
            let mut x = points[i][0];
            let mut y = points[i][1];
            if x < 0 {
                x *= -1;
            }
            if y < 0 {
                y *= -1;
            }
            radius_pair.push((x.max(y), s_vec[i]));
        }
        radius_pair.sort_unstable();

        let mut ans = 0;
        let mut count_hash: HashSet<char> = HashSet::new();

        let mut radius = -1;
        let mut count = 0;

        for i in 0..n {
            if radius_pair[i].0 != radius {
                ans += count;
                count = 0;
                radius = radius_pair[i].0;
            }
            if count_hash.contains(&radius_pair[i].1) {
                return ans;
            }
            count_hash.insert(radius_pair[i].1);
            count += 1;
        }

        ans + count
    }
}

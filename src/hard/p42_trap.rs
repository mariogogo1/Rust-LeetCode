/**
42. 接雨水

给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。

Hint:
n == height.length
1 <= n <= 2 * 104
0 <= height[i] <= 105

https://leetcode.cn/problems/trapping-rain-water/description/
*/
/// 雙指針，移動比較短的指針，水的面積一定是由最短的那一根決定的
pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut lp = 0;
        let mut rp = height.len() - 1;
        let mut ans = 0;
        let mut lp_height = height[lp];
        let mut rp_height = height[rp];
        while lp < rp {
            if lp_height < rp_height {
                ans += lp_height - height[lp];
                lp += 1;
                lp_height = lp_height.max(height[lp]);
            } else {
                ans += rp_height - height[rp];
                rp -= 1;
                rp_height = rp_height.max(height[rp]);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
    #[test]
    fn test_case() {}
}

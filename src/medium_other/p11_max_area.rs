/**
11. 盛最多水的容器

给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i]) 。

找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。

返回容器可以储存的最大水量。

Hint:
n == height.length
2 <= n <= 105
0 <= height[i] <= 104

https://leetcode.cn/problems/container-with-most-water/description/
*/
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut lp = 0;
        let mut rp = height.len() - 1;
        let mut ans = 0;
        while lp < rp {
            let cur_area;
            if height[lp] < height[rp] {
                cur_area = height[lp] * (rp - lp) as i32;
                lp += 1;
            } else {
                cur_area = height[rp] * (rp - lp) as i32;
                rp -= 1;
            }
            ans = ans.max(cur_area);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
    #[test]
    fn test_case() {}
}

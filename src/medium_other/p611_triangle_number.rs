/**
611. 有效三角形的个数

给定一个包含非负整数的数组 nums ，返回其中可以组成三角形三条边的三元组个数。

https://leetcode.cn/problems/valid-triangle-number/description/
*/
pub struct Solution;
impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;
        for i in (2..nums.len()).rev() {
            let mut min_edge = 0 as usize;
            let mut midd_edge = i - 1;

            while min_edge < i - 1 {
                while min_edge < midd_edge && nums[midd_edge] + nums[min_edge] > nums[i] {
                    midd_edge -= 1;
                }

                if midd_edge <= min_edge {
                    ans += (i - min_edge - 1);
                } else {
                    ans += (i - midd_edge - 1);
                }

                min_edge += 1;
            }
        }
        ans as i32
    }
}

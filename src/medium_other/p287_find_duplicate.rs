/**
287. 寻找重复数

给定一个包含 n + 1 个整数的数组 nums ，其数字都在 [1, n] 范围内（包括 1 和 n），可知至少存在一个重复的整数。

假设 nums 只有 一个重复的整数 ，返回 这个重复的数 。

你设计的解决方案必须 不修改 数组 nums 且只用常量级 O(1) 的额外空间。

https://leetcode.cn/problems/find-the-duplicate-number/description/
*/

pub struct Solution;
// 把當前數組當作哈希表使用
impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let mut idx = nums[i];
            if idx < 0 {
                idx *= -1;
            }
            if nums[idx as usize] < 0 {
                return idx;
            }

            nums[idx as usize] *= -1;
        }
        0
    }
}

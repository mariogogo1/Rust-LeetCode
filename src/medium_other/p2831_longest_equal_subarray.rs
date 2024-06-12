/**
2831. 找出最长等值子数组

给你一个下标从 0 开始的整数数组 nums 和一个整数 k 。

如果子数组中所有元素都相等，则认为子数组是一个 等值子数组 。注意，空数组是 等值子数组 。

从 nums 中删除最多 k 个元素后，返回可能的最长等值子数组的长度。

子数组 是数组中一个连续且可能为空的元素序列。

https://leetcode.cn/problems/find-the-longest-equal-subarray/description/
*/

pub struct Solution;
impl Solution {
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut idx_lists = vec![vec![]; nums.len() + 1];
        for i in 0..nums.len() {
            idx_lists[nums[i] as usize].push(i);
        }

        let mut ans = 0;
        for idx_list in idx_lists {
            let mut left = 0 as usize;
            for right in 0..idx_list.len() {
                while idx_list[right] + left > k + idx_list[left] + right {
                    left += 1;
                }
                ans = ans.max(right - left + 1);
            }
        }

        ans as i32
    }
}

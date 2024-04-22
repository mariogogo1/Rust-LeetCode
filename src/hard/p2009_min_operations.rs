/**

2009. 使数组连续的最少操作数

给你一个整数数组 nums 。每一次操作中，你可以将 nums 中 任意 一个元素替换成 任意 整数。

如果 nums 满足以下条件，那么它是 连续的 ：

nums 中所有元素都是 互不相同 的。
nums 中 最大 元素与 最小 元素的差等于 nums.length - 1 。
比方说，nums = [4, 2, 5, 3] 是 连续的 ，但是 nums = [1, 2, 3, 5, 6] 不是连续的 。

请你返回使 nums 连续 的 最少 操作次数。

https://leetcode.cn/problems/minimum-number-of-operations-to-make-array-continuous/description/?envType=daily-question&envId=2024-04-20
*/
pub struct Solution;
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut count = 1;
        let mut f_idx = 1 as usize;
        let n = nums.len() as i32;
        let mut s_idx = 0 as usize;
        let mut ans = i32::MAX;

        while f_idx < nums.len() {
            if nums[f_idx] != nums[f_idx - 1] {
                count += 1;
                while nums[f_idx] - nums[s_idx] >= n {
                    if nums[s_idx] != nums[s_idx + 1] {
                        count -= 1;
                        ans = ans.min(n - count);
                    }
                    s_idx += 1;
                }
            }
            f_idx += 1;
        }
        ans = ans.min(n - count);
        ans
    }
}

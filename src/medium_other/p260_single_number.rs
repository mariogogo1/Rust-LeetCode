/**
260. 只出现一次的数字 III

给你一个整数数组 nums，其中恰好有两个元素只出现一次，其余所有元素均出现两次。 找出只出现一次的那两个元素。你可以按 任意顺序 返回答案。

你必须设计并实现线性时间复杂度的算法且仅使用常量额外空间来解决此问题。

https://leetcode.cn/problems/single-number-iii/description/
*/
pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor_sum = 0;
        for num in &nums {
            xor_sum ^= num;
        }
        let the_least_one = xor_sum & -xor_sum;
        let mut xor_sum_1 = 0;
        let mut xor_sum_2 = 0;

        for num in &nums {
            if num & the_least_one > 0 {
                xor_sum_1 ^= num;
            } else {
                xor_sum_2 ^= num;
            }
        }
        return vec![xor_sum_1, xor_sum_2];
    }
}

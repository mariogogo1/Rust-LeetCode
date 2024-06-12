/**
3151. 特殊数组 I

如果数组的每一对相邻元素都是两个奇偶性不同的数字，则该数组被认为是一个 特殊数组 。

Aging 有一个整数数组 nums。如果 nums 是一个 特殊数组 ，返回 true，否则返回 false。

https://leetcode.cn/problems/special-array-i/description/
*/

pub struct Solution;
impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut if_odd = true;
        if nums[0] % 2 == 0 {
            if_odd = false;
        }
        for i in 1..nums.len() {
            if if_odd {
                if nums[i] % 2 == 0 {
                    if_odd = false;
                } else {
                    return false;
                }
            } else {
                if nums[i] % 2 == 0 {
                    return false;
                } else {
                    if_odd = true;
                }
            }
        }

        true
    }
}

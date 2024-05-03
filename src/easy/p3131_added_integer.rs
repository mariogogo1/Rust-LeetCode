/**
3131. 找出与数组相加的整数 I

给你两个长度相等的数组 nums1 和 nums2。

数组 nums1 中的每个元素都与变量 x 所表示的整数相加。如果 x 为负数，则表现为元素值的减少。

在与 x 相加后，nums1 和 nums2 相等 。当两个数组中包含相同的整数，并且这些整数出现的频次相同时，两个数组 相等 。

返回整数 x 。

https://leetcode.cn/problems/find-the-integer-added-to-array-i/description/
*/
pub struct Solution;

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut min_1 = i32::MAX;
        let mut min_2 = i32::MAX;
        for i in 0..nums1.len() {
            min_1 = min_1.min(nums1[i]);
            min_2 = min_2.min(nums2[i]);
        }
        return min_2 - min_1;
    }
}

/**
100287. 找出与数组相加的整数 II

给你两个整数数组 nums1 和 nums2。

从 nums1 中移除两个元素，并且所有其他元素都与变量 x 所表示的整数相加。如果 x 为负数，则表现为元素值的减少。

执行上述操作后，nums1 和 nums2 相等 。当两个数组中包含相同的整数，并且这些整数出现的频次相同时，两个数组 相等 。

返回能够实现数组相等的 最小 整数 x 。

https://leetcode.cn/problems/find-the-integer-added-to-array-ii/description/
*/
pub struct Solution;
impl Solution {
    pub fn minimum_added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut ans = i32::MAX;
        for i in 0..nums1.len() {
            for j in (i + 1)..nums1.len() {
                let mut ok = true;
                let mut idx = 0;
                while idx == i || idx == j {
                    idx += 1;
                }
                let diff = nums2[0] - nums1[idx];
                for k in 1..nums2.len() {
                    idx += 1;
                    while idx == i || idx == j {
                        idx += 1;
                    }
                    if diff != nums2[k] - nums1[idx] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    ans = ans.min(diff);
                }
            }
        }
        ans
    }
}

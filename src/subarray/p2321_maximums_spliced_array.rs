/**
2321. 拼接数组的最大分数

给你两个下标从 0 开始的整数数组 nums1 和 nums2 ，长度都是 n 。

你可以选择两个整数 left 和 right ，其中 0 <= left <= right < n ，接着 交换 两个子数组 nums1[left...right] 和 nums2[left...right] 。

例如，设 nums1 = [1,2,3,4,5] 和 nums2 = [11,12,13,14,15] ，整数选择 left = 1 和 right = 2，那么 nums1 会变为 [1,12,13,4,5] 而 nums2 会变为 [11,2,3,14,15] 。
你可以选择执行上述操作 一次 或不执行任何操作。

数组的 分数 取 sum(nums1) 和 sum(nums2) 中的最大值，其中 sum(arr) 是数组 arr 中所有元素之和。

返回 可能的最大分数 。

子数组 是数组中连续的一个元素序列。arr[left...right] 表示子数组包含 nums 中下标 left 和 right 之间的元素（含 下标 left 和 right 对应元素）。


https://leetcode.cn/problems/maximum-score-of-spliced-array/description/
*/

/// 先做53 了解 kadane 算法
/// 以兩個數列的差做為新的數列nums1-nums2 = nums3，找尋最大子數組和subarray_max_1，這部分就是可以交換給nums2的部分
/// 可以計算出 如何交換可以使得 "交換後的nums2" 有最大和，sum("交換後的nums2") = subarray_max_1 + sum(nums2)
/// 同理，可以計算 如何交換使得 "交換後的nums1" 有最大和，sum("交換後的nums1") = subarray_max_2 + sum(nums1)
/// 輸出max(sum("交換後的nums2"),sum("交換後的nums1"))

pub struct Solution;
impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut nums1_sum: i32 = 0;
        let mut nums2_sum: i32 = 0;
        let mut current_max_1: i32 = 0;
        let mut current_max_2: i32 = 0;
        let mut subarray_max_1: i32 = i32::MIN;
        let mut subarray_max_2: i32 = i32::MIN;

        for i in 0..n {
            nums1_sum += nums1[i];
            nums2_sum += nums2[i];
            current_max_1 = current_max_1.max(0) + nums1[i] - nums2[i];
            subarray_max_1 = subarray_max_1.max(current_max_1);
            current_max_2 = current_max_2.max(0) - nums1[i] + nums2[i];
            subarray_max_2 = subarray_max_2.max(current_max_2);
        }
        (nums1_sum + subarray_max_2).max(nums2_sum + subarray_max_1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::maximums_spliced_array(vec![60, 60, 60], vec![10, 90, 10]),
            210
        );
        assert_eq!(
            Solution::maximums_spliced_array(vec![20, 40, 20, 70, 30], vec![50, 20, 50, 40, 20]),
            220
        );
        assert_eq!(
            Solution::maximums_spliced_array(vec![7, 11, 13], vec![1, 1, 1]),
            31
        );
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

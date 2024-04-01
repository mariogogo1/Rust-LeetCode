/**
53. 最大子数组和

给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

子数组
是数组中的一个连续部分。
提示：

1 <= nums.length <= 105
-104 <= nums[i] <= 104


进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。

https://leetcode.cn/problems/maximum-subarray/description/
*/
pub struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans: i32 = i32::MIN;
        let mut current_max: i32 = 0;

        for num in nums {
            current_max = current_max.max(0) + num;
            ans = ans.max(current_max);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1]), 1);

        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
    #[test]
    fn test_case_1() {
        assert_eq!(Solution::max_sub_array(vec![-3, -2]), -2);
    }
    #[test]
    fn test_case_2() {}
}

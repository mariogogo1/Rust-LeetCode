/**
152. 乘积最大子数组

给你一个整数数组 nums ，请你找出数组中乘积最大的非空连续
子数组
（该子数组中至少包含一个数字），并返回该子数组所对应的乘积。

测试用例的答案是一个 32-位 整数。
提示:

1 <= nums.length <= 2 * 104
-10 <= nums[i] <= 10
nums 的任何前缀或后缀的乘积都 保证 是一个 32-位 整数


https://leetcode.cn/problems/maximum-product-subarray/description/
*/
pub struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ans: i32 = i32::MIN;
        let mut current_max: i32 = 1;
        let mut current_min: i32 = 1;

        for num in nums {
            if num < 0 {
                (current_max, current_min) = (current_min, current_max);
            }
            current_max = num.max(num * current_max);
            current_min = num.min(num * current_min);
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
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

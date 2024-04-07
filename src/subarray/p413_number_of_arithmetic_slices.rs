/**
413. 等差数列划分

如果一个数列 至少有三个元素 ，并且任意两个相邻元素之差相同，则称该数列为等差数列。

例如，[1,3,5,7,9]、[7,7,7,7] 和 [3,-1,-5,-9] 都是等差数列。
给你一个整数数组 nums ，返回数组 nums 中所有为等差数组的 子数组 个数。

子数组 是数组中的一个连续序列。

https://leetcode.cn/problems/arithmetic-slices/description/
*/
pub struct Solution;
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut arithmetic_length = 2;
        for i in 2..n {
            if (nums[i] - nums[i - 1]) * arithmetic_length
                == nums[i] - nums[i - arithmetic_length as usize]
            {
                arithmetic_length += 1;
                ans += arithmetic_length - 2;
            } else {
                arithmetic_length = 2;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

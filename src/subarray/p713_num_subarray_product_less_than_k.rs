/**
713. 乘积小于 K 的子数组

给你一个整数数组 nums 和一个整数 k ，请你返回子数组内所有元素的乘积严格小于 k 的连续子数组的数目。

https://leetcode.cn/problems/subarray-product-less-than-k/description/
*/

pub struct Solution;
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut left_ptr: usize = 0;
        let mut product: i32 = 1;
        let mut count: i32 = 0;

        for right_ptr in 0..nums.len() {
            product *= nums[right_ptr];
            while product >= k && left_ptr <= right_ptr {
                product /= nums[left_ptr];
                left_ptr += 1;
            }
            if product < k {
                count += (right_ptr - left_ptr + 1) as i32;
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
            0
        );
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

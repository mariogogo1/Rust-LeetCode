/**
209. 长度最小的子数组

给定一个含有 n 个正整数的数组和一个正整数 target 。

找出该数组中满足其总和大于等于 target 的长度最小的 连续
子数组
 [numsl, numsl+1, ..., numsr-1, numsr] ，并返回其长度。如果不存在符合条件的子数组，返回 0 。


https://leetcode.cn/problems/minimum-size-subarray-sum/description/
*/
pub struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left_ptr: usize = 0;
        let mut sum: i32 = 0;
        let mut count: i32 = i32::MAX;

        for right_ptr in 0..nums.len() {
            sum += nums[right_ptr];
            while sum >= target && left_ptr <= right_ptr {
                count = count.min((right_ptr - left_ptr + 1) as i32);
                sum -= nums[left_ptr];
                left_ptr += 1;
            }
        }
        if count == i32::MAX {
            return 0;
        }
        return count;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

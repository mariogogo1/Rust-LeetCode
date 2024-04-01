/**
974. 和可被 K 整除的子数组

给定一个整数数组 nums 和一个整数 k ，返回其中元素之和可被 k 整除的（连续、非空） 子数组 的数目。

子数组 是数组的 连续 部分。
提示:

1 <= nums.length <= 3 * 104
-104 <= nums[i] <= 104
2 <= k <= 104

https://leetcode.cn/problems/subarray-sums-divisible-by-k/description/
*/
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix_sum: i32 = 0;
        let mut sum_mod_hashmap: HashMap<i32, i32> = HashMap::new();
        let mut ans: i32 = 0;
        sum_mod_hashmap.insert(0, 1);
        for num in nums {
            prefix_sum += num;
            let mut mod_value = prefix_sum % k;
            while mod_value < 0 {
                mod_value += k;
            }
            if let Some(&s) = sum_mod_hashmap.get(&(mod_value)) {
                ans += s;
            }
            *sum_mod_hashmap.entry(mod_value).or_insert(0) += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    }
    #[test]
    fn test_case_1() {
        assert_eq!(Solution::subarrays_div_by_k(vec![5], 9), 0);
    }
    #[test]
    fn test_case_2() {}
}

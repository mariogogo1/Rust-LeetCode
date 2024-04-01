/**
523. 连续的子数组和

给你一个整数数组 nums 和一个整数 k ，编写一个函数来判断该数组是否含有同时满足下述条件的连续子数组：

子数组大小 至少为 2 ，且
子数组元素总和为 k 的倍数。
如果存在，返回 true ；否则，返回 false 。

如果存在一个整数 n ，令整数 x 符合 x = n * k ，则称 x 是 k 的一个倍数。0 始终视为 k 的一个倍数。
提示：

1 <= nums.length <= 105
0 <= nums[i] <= 109
0 <= sum(nums[i]) <= 231 - 1
1 <= k <= 231 - 1

https://leetcode.cn/problems/continuous-subarray-sum/description/
*/
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut prefix_sum: i32 = 0;
        let mut sum_mod_hashmap: HashMap<i32, i32> = HashMap::new();
        sum_mod_hashmap.insert(0, 1);
        for num in nums {
            prefix_sum += num;
            let mod_value = prefix_sum % k;
            if let Some(&s) = sum_mod_hashmap.get(&(mod_value)) {
                if s >= 2 || (s == 1 && num % k != 0) {
                    return true;
                }
            }
            *sum_mod_hashmap.entry(mod_value).or_insert(0) += 1;
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
    }
    #[test]
    fn test_case_1() {
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
    }
    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 23),
            false
        );
    }
}

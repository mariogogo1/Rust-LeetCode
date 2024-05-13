/**
3115. 素数的最大距离

给你一个整数数组 nums。

返回两个（不一定不同的）素数在 nums 中 下标 的 最大距离。

提示：

1 <= nums.length <= 3 * 105
1 <= nums[i] <= 100
输入保证 nums 中至少有一个素数

https://leetcode.cn/problems/maximum-prime-difference/description/
*/
pub struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let mut prime_hashset: HashSet<i32> = HashSet::new();
        let mut non_prime_hashset: HashSet<i32> = HashSet::new();
        let mut max_index = 0;
        let mut min_index = 0;
        non_prime_hashset.insert(1);

        for i in 2..=100 {
            if !non_prime_hashset.contains(&i) {
                prime_hashset.insert(i);
                let mut j = 2;
                while i * j <= 100 {
                    non_prime_hashset.insert(i * j);
                    j += 1
                }
            }
        }

        for i in 0..nums.len() {
            if prime_hashset.contains(&nums[i]) {
                min_index = i as i32;
                break;
            }
        }

        for i in ((min_index as usize)..nums.len()).rev() {
            if prime_hashset.contains(&nums[i]) {
                max_index = i as i32;
                break;
            }
        }

        max_index - min_index
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case() {}
}

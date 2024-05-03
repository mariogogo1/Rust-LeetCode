/**
2799. 统计完全子数组的数目

给你一个由 正 整数组成的数组 nums 。

如果数组中的某个子数组满足下述条件，则称之为 完全子数组 ：

子数组中 不同 元素的数目等于整个数组不同元素的数目。
返回数组中 完全子数组 的数目。

子数组 是数组中的一个连续非空序列。

https://leetcode.cn/problems/count-complete-subarrays-in-an-array/description/
*/
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut hash_count = HashMap::new();
        for &num in nums.iter() {
            *hash_count.entry(num).or_insert(0) += 1;
        }
        let n = nums.len();
        let count = hash_count.len();
        let mut ans = 0;
        let mut r = 0 as usize;
        hash_count.clear();
        for l in 0..n {
            if l > 0 {
                let mut val = hash_count.entry(nums[l - 1]).or_insert(0);

                *val -= 1;

                if *val == 0 {
                    hash_count.remove(&nums[l - 1]);
                }
            }
            if count > hash_count.len() {
                while r < nums.len() {
                    *hash_count.entry(nums[r]).or_insert(0) += 1;
                    r += 1;
                    if count == hash_count.len() {
                        ans += (n - r + 1) as i32;
                        break;
                    }
                }
            } else {
                ans += (n - r + 1) as i32;
            }

            if count > hash_count.len() {
                break;
            }
        }

        ans
    }
}

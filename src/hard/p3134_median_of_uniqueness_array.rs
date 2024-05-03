/**
3134. 找出唯一性数组的中位数

给你一个整数数组 nums 。数组 nums 的 唯一性数组 是一个按元素从小到大排序的数组，包含了 nums 的所有
非空子数组中
不同元素的个数。

换句话说，这是由所有 0 <= i <= j < nums.length 的 distinct(nums[i..j]) 组成的递增数组。

其中，distinct(nums[i..j]) 表示从下标 i 到下标 j 的子数组中不同元素的数量。

返回 nums 唯一性数组 的 中位数 。

注意，数组的 中位数 定义为有序数组的中间元素。如果有两个中间元素，则取值较小的那个。

https://leetcode.cn/problems/find-the-median-of-the-uniqueness-array/description/
*/
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut is_odd_median = false;
        if n % 4 == 1 || n % 4 == 2 {
            is_odd_median = true;
        }
        let key = if is_odd_median {
            n * (n + 1) / 4 + 1
        } else {
            n * (n + 1) / 4
        };

        let mut l = 1 as usize;
        let mut r = n as usize;

        // 二分查找 有多少的DISJOINT[i,j]的大小 <= 中位數m
        while l < r {
            let m = l + (r - l) / 2;
            let count_median = Self::get_count_median(&nums, m);

            if count_median >= key {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as i32
    }

    // 使用滑動窗口
    fn get_count_median(nums: &Vec<i32>, m: usize) -> usize {
        let mut count_hash: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0 as usize;
        let mut l = 0 as usize;
        for r in 0..nums.len() {
            *count_hash.entry(nums[r]).or_insert(0) += 1;
            if count_hash.len() <= m {
                ans += r - l + 1;
            } else {
                while count_hash.len() > m {
                    let mut val = count_hash.entry(nums[l]).or_insert(0);
                    *val -= 1;
                    if *val == 0 {
                        count_hash.remove(&nums[l]);
                    }
                    l += 1;
                }
                ans += r - l + 1;
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
        assert_eq!(Solution::median_of_uniqueness_array(vec![1, 2, 3]), 1);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::median_of_uniqueness_array(vec![3, 4, 3, 4, 5]), 2);
        assert_eq!(Solution::median_of_uniqueness_array(vec![4, 3, 5, 4]), 2);
    }
}

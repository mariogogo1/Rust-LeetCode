/**

446. 等差数列划分 II - 子序列

给你一个整数数组 nums ，返回 nums 中所有 等差子序列 的数目。

如果一个序列中 至少有三个元素 ，并且任意两个相邻元素之差相同，则称该序列为等差序列。

例如，[1, 3, 5, 7, 9]、[7, 7, 7, 7] 和 [3, -1, -5, -9] 都是等差序列。
再例如，[1, 1, 2, 5, 7] 不是等差序列。
数组中的子序列是从数组中删除一些元素（也可能不删除）得到的一个序列。

例如，[2,5,10] 是 [1,2,1,2,4,1,5,10] 的一个子序列。
题目数据保证答案是一个 32-bit 整数。

提示：

1  <= nums.length <= 1000
-231 <= nums[i] <= 231 - 1

https://leetcode.cn/problems/arithmetic-slices-ii-subsequence/description/
*/
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut diff_hashmaps: Vec<HashMap<i64, i32>> = vec![HashMap::new(); n];
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64; // 要小心題目給的範圍
                if let Some(&count) = diff_hashmaps[j].get(&diff) {
                    *diff_hashmaps[i].entry(diff).or_insert(0) += count;
                    ans += count;
                }
                *diff_hashmaps[i].entry(diff).or_insert(0) += 1;
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
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]),
            7
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]),
            16
        );
    }
    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296]),
            0
        );
    }
    #[test]
    fn test_case_2() {}
}

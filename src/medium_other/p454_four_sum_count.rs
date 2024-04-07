/**
454. 四数相加 II

给你四个整数数组 nums1、nums2、nums3 和 nums4 ，数组长度都是 n ，请你计算有多少个元组 (i, j, k, l) 能满足：

0 <= i, j, k, l < n
nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0

 提示：

n == nums1.length
n == nums2.length
n == nums3.length
n == nums4.length
1 <= n <= 200
-228 <= nums1[i], nums2[i], nums3[i], nums4[i] <= 228

https://leetcode.cn/problems/4sum-ii/description/
*/
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        for &a in &nums1 {
            for &b in &nums2 {
                *hashmap.entry(-1 * (a + b)).or_insert(0) += 1;
            }
        }
        let mut ans = 0;
        for &c in &nums3 {
            for &d in &nums4 {
                if let Some(&value) = hashmap.get(&(c + d)) {
                    ans += value;
                }
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
            Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
            2
        );
    }
    #[test]
    fn test_case() {}
}

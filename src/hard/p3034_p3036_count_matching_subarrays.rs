/**

3034. 匹配模式数组的子数组数目 I
3036. 匹配模式数组的子数组数目 II
给你一个下标从 0 开始长度为 n 的整数数组 nums ，和一个下标从 0 开始长度为 m 的整数数组 pattern ，pattern 数组只包含整数 -1 ，0 和 1 。

大小为 m + 1 的
子数组
 nums[i..j] 如果对于每个元素 pattern[k] 都满足以下条件，那么我们说这个子数组匹配模式数组 pattern ：

如果 pattern[k] == 1 ，那么 nums[i + k + 1] > nums[i + k]
如果 pattern[k] == 0 ，那么 nums[i + k + 1] == nums[i + k]
如果 pattern[k] == -1 ，那么 nums[i + k + 1] < nums[i + k]
请你返回匹配 pattern 的 nums 子数组的 数目 。

提示：

2 <= n == nums.length <= 100
1 <= nums[i] <= 109
1 <= m == pattern.length < n
-1 <= pattern[i] <= 1

https://leetcode.cn/problems/number-of-subarrays-that-match-a-pattern-i/description/
https://leetcode.cn/problems/number-of-subarrays-that-match-a-pattern-ii/description/
*/
pub struct Solution;

/// KMP算法
impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let mut nums_pattern = vec![0; nums.len() - 1];
        for i in 0..nums_pattern.len() {
            if nums[i + 1] > nums[i] {
                nums_pattern[i] = 1;
            } else if nums[i + 1] == nums[i] {
                nums_pattern[i] = 0;
            } else {
                nums_pattern[i] = -1;
            }
        }

        // KMP 失效函數 failure
        let mut j = 0;
        let mut failure = vec![0; pattern.len()];
        for i in 1..pattern.len() {
            while j > 0 && pattern[i] != pattern[j] {
                j = failure[j - 1];
            }
            if pattern[i] == pattern[j] {
                j += 1;
            }
            failure[i] = j;
        }

        let mut count = 0;
        j = 0;
        for i in 0..nums_pattern.len() {
            while j > 0 && nums_pattern[i] != pattern[j] {
                j = failure[j - 1];
            }
            if nums_pattern[i] == pattern[j] {
                j += 1;
                if j == pattern.len() {
                    count += 1;
                    j = failure[j - 1];
                }
            }
        }

        count
    }
}

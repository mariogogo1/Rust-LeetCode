/**
496. 下一个更大元素 I

nums1 中数字 x 的 下一个更大元素 是指 x 在 nums2 中对应位置 右侧 的 第一个 比 x 大的元素。

给你两个 没有重复元素 的数组 nums1 和 nums2 ，下标从 0 开始计数，其中nums1 是 nums2 的子集。

对于每个 0 <= i < nums1.length ，找出满足 nums1[i] == nums2[j] 的下标 j ，并且在 nums2 确定 nums2[j] 的 下一个更大元素 。如果不存在下一个更大元素，那么本次查询的答案是 -1 。

返回一个长度为 nums1.length 的数组 ans 作为答案，满足 ans[i] 是如上所述的 下一个更大元素 。

https://leetcode.cn/problems/next-greater-element-i/description/
*/

pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack_nums: Vec<i32> = Vec::new();
        let mut nums_hashmap: HashMap<i32, i32> = HashMap::new();
        let mut ans: Vec<i32> = vec![-1; nums1.len()];

        // Build a hashmap of the next greater elements in nums2
        for &num in nums2.iter().rev() {
            // 出現NONE不會進入WHILE
            while let Some(&top) = stack_nums.last() {
                if top <= num {
                    stack_nums.pop();
                } else {
                    nums_hashmap.insert(num, top);
                    break;
                }
            }
            stack_nums.push(num);
        }

        // Fill the answer vector using the hashmap
        for (i, &num) in nums1.iter().enumerate() {
            // 出現NONE不會進入IF
            if let Some(&next_greater) = nums_hashmap.get(&num) {
                ans[i] = next_greater;
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
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
    #[test]
    fn test_case() {}
}

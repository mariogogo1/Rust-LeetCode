/**
456. 132 模式

给你一个整数数组 nums ，数组中共有 n 个整数。132 模式的子序列 由三个整数 nums[i]、nums[j] 和 nums[k] 组成，并同时满足：i < j < k 和 nums[i] < nums[k] < nums[j] 。

如果 nums 中存在 132 模式的子序列 ，返回 true ；否则，返回 false 。

提示：

n == nums.length
1 <= n <= 2 * 105
-109 <= nums[i] <= 109

https://leetcode.cn/problems/132-pattern/description/
*/

pub struct Solution;
//456. 132 模式
/// 思考模式類似 768 如何有最多的塊
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut stack: Vec<usize> = Vec::new();
        let mut second = i32::MIN;

        for i in (0..n).rev() {
            if nums[i] < second {
                return true;
            }
            while let Some(&top) = stack.last() {
                if nums[top] < nums[i] {
                    second = nums[top];
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        return false;
    }

    pub fn find132pattern_basic(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut stack: Vec<usize> = Vec::new();
        let mut min_stack: Vec<usize> = Vec::new();
        let mut cur_min_index: usize = 0;

        for i in 0..n {
            if nums[cur_min_index] > nums[i] {
                cur_min_index = i;
            }
            while let Some(&top) = stack.last() {
                if nums[top] <= nums[i] {
                    stack.pop();
                    min_stack.pop();
                } else {
                    if let Some(&min_top) = min_stack.last() {
                        if nums[min_top] < nums[i] {
                            return true;
                        }
                    }
                    break;
                }
            }
            stack.push(i);
            min_stack.push(cur_min_index);
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::find132pattern(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::find132pattern(vec![3, 1, 4, 2]), true);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::find132pattern(vec![3, 5, 0, 3, 4]), true);
        assert_eq!(Solution::find132pattern(vec![7, 9, 4, 6, 0, 2, 3]), false);
        assert_eq!(Solution::find132pattern(vec![7, 9, 4, 6, 0, 2, 1]), true);
    }
}

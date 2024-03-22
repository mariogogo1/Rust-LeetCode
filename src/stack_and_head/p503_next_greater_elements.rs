/**
503. 下一个更大元素 II

给定一个循环数组 nums （ nums[nums.length - 1] 的下一个元素是 nums[0] ），返回 nums 中每个元素的 下一个更大元素 。

数字 x 的 下一个更大的元素 是按数组遍历顺序，这个数字之后的第一个比它更大的数，这意味着你应该循环地搜索它的下一个更大的数。如果不存在，则输出 -1 。

https://leetcode.cn/problems/next-greater-element-ii/description/
*/

pub struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack_index: Vec<usize> = Vec::new();
        let mut ans: Vec<i32> = vec![-1; nums.len()];

        // 循环两次以考虑环形数组
        for _ in 0..2 {
            for (j, &num) in nums.iter().enumerate() {
                while let Some(&top_index) = stack_index.last() {
                    // 修改比较逻辑，如果当前数字比栈顶索引对应的数字大，则更新结果并弹出栈顶元素
                    if num > nums[top_index] {
                        ans[top_index] = num;
                        stack_index.pop();
                    } else {
                        break;
                    }
                }
                stack_index.push(j);
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
            Solution::next_greater_elements(vec![1, 2, 1]),
            vec![2, -1, 2]
        );
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 3, 4, 3]),
            vec![2, 3, 4, -1, 4]
        );
    }
    #[test]
    fn test_case() {}
}

/**
2454. 下一个更大元素 IV

给你一个下标从 0 开始的非负整数数组 nums 。对于 nums 中每一个整数，你必须找到对应元素的 第二大 整数。

如果 nums[j] 满足以下条件，那么我们称它为 nums[i] 的 第二大 整数：

j > i
nums[j] > nums[i]
恰好存在 一个 k 满足 i < k < j 且 nums[k] > nums[i] 。
如果不存在 nums[j] ，那么第二大整数为 -1 。

比方说，数组 [1, 2, 4, 3] 中，1 的第二大整数是 4 ，2 的第二大整数是 3 ，3 和 4 的第二大整数是 -1 。
请你返回一个整数数组 answer ，其中 answer[i]是 nums[i] 的第二大整数。

https://leetcode.cn/problems/next-greater-element-iv/description/
*/

pub struct Solution;
impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let mut first_stack: Vec<usize> = Vec::new();
        let mut second_stack: Vec<usize> = Vec::new();

        let mut ans: Vec<i32> = vec![-1; nums.len()];
        for i in 0..nums.len() {
            while let Some(&top_2) = second_stack.last() {
                if nums[top_2] < nums[i] {
                    ans[top_2] = nums[i]; // 找到第二個較大的元素
                    second_stack.pop();
                } else {
                    break;
                }
            }
            let mut count = 0;
            for j in (0..first_stack.len()).rev() {
                if nums[first_stack[j]] < nums[i] {
                    count += 1;
                } else {
                    break;
                }
            }
            // 找到第一個較大的元素，整段移動到second_stack，為了順序保持不變
            // 但是可能調用資源比較多
            second_stack.extend(first_stack.split_off(first_stack.len() - count));
            first_stack.push(i);
        }
        return ans;
    }
    pub fn second_greater_element_basic(nums: Vec<i32>) -> Vec<i32> {
        let mut first_stack: Vec<usize> = Vec::new();
        let mut second_stack: Vec<usize> = Vec::new();
        let mut temp_stack: Vec<usize> = Vec::new();

        let mut ans: Vec<i32> = vec![-1; nums.len()];
        for i in 0..nums.len() {
            while let Some(&top_2) = second_stack.last() {
                if nums[top_2] < nums[i] {
                    ans[top_2] = nums[i]; // 找到第二個較大的元素
                    second_stack.pop();
                } else {
                    break;
                }
            }
            while let Some(&top_1) = first_stack.last() {
                if nums[top_1] < nums[i] {
                    first_stack.pop();
                    temp_stack.push(top_1); // 找到第一個較大的元素，整段移動到second_stack，為了順序保持不變，多做一個temp_stack轉乘
                } else {
                    break;
                }
            }
            while let Some(&top) = temp_stack.last() {
                temp_stack.pop();
                second_stack.push(top);
            }
            first_stack.push(i);
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::second_greater_element(vec![2, 4, 0, 9, 6]),
            vec![9, 6, 6, -1, -1]
        );
        assert_eq!(Solution::second_greater_element(vec![3, 3]), vec![-1, -1]);
    }
    #[test]
    fn test_case() {}
}

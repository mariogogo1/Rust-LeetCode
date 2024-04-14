/**
31. 下一个排列

整数数组的一个 排列  就是将其所有成员以序列或线性顺序排列。

例如，arr = [1,2,3] ，以下这些都可以视作 arr 的排列：[1,2,3]、[1,3,2]、[3,1,2]、[2,3,1] 。
整数数组的 下一个排列 是指其整数的下一个字典序更大的排列。更正式地，如果数组的所有排列根据其字典顺序从小到大排列在一个容器中，那么数组的 下一个排列 就是在这个有序容器中排在它后面的那个排列。如果不存在下一个更大的排列，那么这个数组必须重排为字典序最小的排列（即，其元素按升序排列）。

例如，arr = [1,2,3] 的下一个排列是 [1,3,2] 。
类似地，arr = [2,3,1] 的下一个排列是 [3,1,2] 。
而 arr = [3,2,1] 的下一个排列是 [1,2,3] ，因为 [3,2,1] 不存在一个字典序更大的排列。
给你一个整数数组 nums ，找出 nums 的下一个排列。

必须 原地 修改，只允许使用额外常数空间。

https://leetcode.cn/problems/next-permutation/description/
*/
pub struct Solution;

///
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut stack = Vec::new();
        for i in (0..n).rev() {
            if let Some(&value) = stack.last() {
                if nums[i] < value {
                    break;
                }
            }
            stack.push(nums[i]);
            nums.pop();
        }

        let m = stack.len();

        if let Some(last) = nums.last_mut() {
            for i in 0..m {
                if stack[i] > *last {
                    let a = stack[i];
                    stack[i] = *last;
                    *last = a;
                    break;
                }
            }
        }
        nums.extend(stack);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        //      assert_eq!(Solution::next_permutation(&mut vec![1, 2, 3]), ());
    }
    #[test]
    fn test_case() {}
}

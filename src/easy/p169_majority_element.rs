/**
169. 多数元素

给定一个大小为 n 的数组 nums ，返回其中的多数元素。多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。

你可以假设数组是非空的，并且给定的数组总是存在多数元素。
https://leetcode.cn/problems/majority-element/description/
*/

pub struct Solution;

///Boyer-Moore 投票算法
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut ans = -1;
        for num in nums {
            if count == 0 {
                ans = num;
            }
            if ans == num {
                count += 1;
            } else {
                count -= 1;
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
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
    #[test]
    fn test_case() {}
}

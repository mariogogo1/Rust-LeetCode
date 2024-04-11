/**
3101. 交替子数组计数

给你一个
二进制数组
nums 。

如果一个
子数组
中 不存在 两个 相邻 元素的值 相同 的情况，我们称这样的子数组为 交替子数组 。

返回数组 nums 中交替子数组的数量

https://leetcode.cn/problems/count-alternating-subarrays/description/
*/

pub struct Solution;
impl Solution {
    /// 標記出現兩個相同相鄰元素的位置，每次出現就表示有n*(n+1)/2的子數組滿足條件。
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let mut index: usize = 0;
        let mut ans: i64 = 0;

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                ans += ((i - index + 1) as i64) * ((i - index) as i64) / 2;
                index = i;
            }
        }
        ans += ((nums.len() - index + 1) as i64) * ((nums.len() - index) as i64) / 2;
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::count_alternating_subarrays(vec![1, 0, 1, 0]), 10);
        assert_eq!(Solution::count_alternating_subarrays(vec![0, 1, 1, 1]), 5);
    }
    #[test]
    fn test_case() {}
}

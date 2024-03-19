/**
2899. 上一个遍历的整数

给你一个整数数组 nums ，其中 nums[i] 要么是一个正整数，要么是 -1 。

我们需要为每个 -1 找到相应的正整数，我们称之为最后访问的整数。

为了达到这个目标，定义两个空数组：seen 和 ans。

从数组 nums 的头部开始遍历。

如果遇到正整数，把它添加到 seen 的 头部。
如果遇到 -1，则设 k 是到目前为止看到的 连续 -1 的数目(包括当前 -1)，
如果 k 小于等于 seen 的长度，把 seen 的第 k 个元素添加到 ans。
如果 k 严格大于 seen 的长度，把 -1 添加到 ans。
请你返回数组 ans。

https://leetcode.cn/problems/last-visited-integers/description/
*/
pub struct Solution;

impl Solution {
    pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();
        let mut ans: Vec<i32> = Vec::new();
        let mut ptr = 0;
        for (i, &x) in nums.iter().enumerate() {
            if x > 0 {
                stack.push(x);
                ptr = stack.len();
            } else {
                if ptr > 0 {
                    ans.push(stack[ptr - 1]);
                    ptr -= 1;
                } else {
                    ans.push(-1);
                };
            };
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
            Solution::last_visited_integers(vec![1, 2, -1, -1, -1],),
            vec![2, 1, -1]
        );
        assert_eq!(
            Solution::last_visited_integers(vec![1, -1, 2, -1, -1]),
            vec![1, 2, 1]
        );
    }
    #[test]
    fn test_case() {}
}

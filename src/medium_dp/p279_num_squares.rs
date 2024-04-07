/**
279. 完全平方数

给你一个整数 n ，返回 和为 n 的完全平方数的最少数量 。

完全平方数 是一个整数，其值等于另一个整数的平方；换句话说，其值等于一个整数自乘的积。例如，1、4、9 和 16 都是完全平方数，而 3 和 11 不是。

 https://leetcode.cn/problems/perfect-squares/description/
*/
pub struct Solution;
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp: Vec<i32> = vec![0; n + 1];
        for i in 1..n + 1 {
            let mut j = 1;
            let mut ans = i32::MAX;
            while j * j <= i {
                ans = ans.min(dp[i - j * j]);
                j += 1;
            }
            dp[i] = ans + 1;
        }
        dp[n]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
    }
    #[test]
    fn test_case() {}
}

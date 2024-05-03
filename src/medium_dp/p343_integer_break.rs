/**
343. 整数拆分

给定一个正整数 n ，将其拆分为 k 个 正整数 的和（ k >= 2 ），并使这些整数的乘积最大化。

返回 你可以获得的最大乘积 。

https://leetcode.cn/problems/integer-break/description/
*/
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 || n == 3 {
            return n - 1;
        }

        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            let mut ans = 0;
            for j in 1..=n {
                if i >= j {
                    ans = ans.max(dp[i - j] * (j));
                }
            }
            dp[i] = ans;
        }

        dp[n] as i32
    }
}

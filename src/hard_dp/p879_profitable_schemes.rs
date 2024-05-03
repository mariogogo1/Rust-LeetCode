/**
879. 盈利计划

集团里有 n 名员工，他们可以完成各种各样的工作创造利润。

第 i 种工作会产生 profit[i] 的利润，它要求 group[i] 名成员共同参与。如果成员参与了其中一项工作，就不能参与另一项工作。

工作的任何至少产生 minProfit 利润的子集称为 盈利计划 。并且工作的成员总数最多为 n 。

有多少种计划可以选择？因为答案很大，所以 返回结果模 10^9 + 7 的值。

https://leetcode.cn/problems/profitable-schemes/description/
*/
pub struct Solution;

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = n as usize;
        let min_profit = min_profit as usize;
        let len = group.len();
        let mut dp = vec![vec![0; n + 1]; min_profit + 1];
        dp[0][0] = 1;

        for key in 0..len {
            let p = profit[key] as usize;
            for i in (0..=min_profit).rev() {
                for j in (0..=n).rev() {
                    if dp[i][j] > 0 {
                        let people = j + group[key] as usize;
                        let mut this_profit = i + p;
                        if this_profit >= min_profit {
                            this_profit = min_profit;
                        }
                        if people <= n {
                            dp[this_profit][people] += dp[i][j];
                            dp[this_profit][people] %= 1_000_000_007;
                        }
                    }
                }
            }
        }

        let mut ans = 0;
        for &value in dp[min_profit].iter() {
            ans += value;
            ans %= 1_000_000_007;
        }
        ans as i32
    }
}

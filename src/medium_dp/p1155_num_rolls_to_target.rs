/**
1155. 掷骰子等于目标和的方法数

这里有 n 个一样的骰子，每个骰子上都有 k 个面，分别标号为 1 到 k 。

给定三个整数 n、k 和 target，请返回投掷骰子的所有可能得到的结果（共有 kn 种方式），使得骰子面朝上的数字总和等于 target。

由于答案可能很大，你需要对 109 + 7 取模。

https://leetcode.cn/problems/number-of-dice-rolls-with-target-sum/description/
*/
/// 背包
impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let target = target as usize;
        let k = k as usize;
        let n = n as usize;
        let mut dp = vec![vec![0 as i64; 1 + target]; n + 1];
        let mut prefix = vec![1 as i64; 1 + target];
        dp[0][0] = 1;
        for i in 1..=n {
            for j in 1..=target {
                dp[i][j] += prefix[j - 1];
                if j >= k + 1 {
                    dp[i][j] -= prefix[j - k - 1];
                }
                dp[i][j] %= Self::VAL_MOD;
            }
            prefix[0] = 0;
            for j in 1..=target {
                prefix[j] = prefix[j - 1] + dp[i][j];
            }
        }
        dp[n][target] as i32
    }
}

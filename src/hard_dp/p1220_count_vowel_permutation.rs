/**

1220. 统计元音字母序列的数目

给你一个整数 n，请你帮忙统计一下我们可以按下述规则形成多少个长度为 n 的字符串：

字符串中的每个字符都应当是小写元音字母（'a', 'e', 'i', 'o', 'u'）
每个元音 'a' 后面都只能跟着 'e'
每个元音 'e' 后面只能跟着 'a' 或者是 'i'
每个元音 'i' 后面 不能 再跟着另一个 'i'
每个元音 'o' 后面只能跟着 'i' 或者是 'u'
每个元音 'u' 后面只能跟着 'a'
由于答案可能会很大，所以请你返回 模 10^9 + 7 之后的结果。

https://leetcode.cn/problems/count-vowels-permutation/description/
*/
pub struct Solution;
/// DP[i][j] 長度為j的字串，第i種字母做最後一個字母時有幾種可能
impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn count_vowel_permutation(n: i32) -> i32 {
        let n = n as usize;
        let mut dp: Vec<Vec<i64>> = vec![vec![0; n]; 5];
        let mut ans: i64 = 0;

        for i in 0..5 {
            dp[i][0] = 1;
        }

        for j in 0..(n - 1) {
            dp[1][j + 1] += dp[0][j];
            dp[0][j + 1] += dp[1][j];
            dp[2][j + 1] += dp[1][j];
            dp[0][j + 1] += dp[2][j];
            dp[1][j + 1] += dp[2][j];
            dp[3][j + 1] += dp[2][j];
            dp[4][j + 1] += dp[2][j];
            dp[2][j + 1] += dp[3][j];
            dp[4][j + 1] += dp[3][j];
            dp[0][j + 1] += dp[4][j];

            dp[0][j + 1] %= Self::VAL_MOD;
            dp[1][j + 1] %= Self::VAL_MOD;
            dp[2][j + 1] %= Self::VAL_MOD;
            dp[3][j + 1] %= Self::VAL_MOD;
            dp[4][j + 1] %= Self::VAL_MOD;
        }

        for i in 0..5 {
            ans += dp[i][n - 1];
            ans %= Self::VAL_MOD;
        }
        ans as i32
    }
}

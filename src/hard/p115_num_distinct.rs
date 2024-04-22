/**

115. 不同的子序列

给你两个字符串 s 和 t ，统计并返回在 s 的 子序列 中 t 出现的个数，结果需要对 109 + 7 取模。

https://leetcode.cn/problems/distinct-subsequences/description/
*/
pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let n = s.len();
        let m = t.len();

        if n >= m {
            // nth 是每次疊帶去尋找 會產生不必要的消耗 先改成VEC
            // t.chars().nth(i - 1).unwrap() == s.chars().nth(j - 1).unwrap()
            let s_chars: Vec<char> = s.chars().collect();
            let t_chars: Vec<char> = t.chars().collect();

            let mut dp = vec![vec![0; n + 1]; m + 1];
            for j in 0..=n {
                dp[0][j] = 1;
            }
            for i in 1..=m {
                for j in 1..=n {
                    dp[i][j] = dp[i][j - 1];
                    if t_chars[i - 1] == s_chars[j - 1] {
                        dp[i][j] += dp[i - 1][j - 1];
                    }
                }
            }
            return dp[m][n];
        }
        0
    }
}

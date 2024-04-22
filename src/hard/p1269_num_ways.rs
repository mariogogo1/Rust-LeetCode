/**
1269. 停在原地的方案数

有一个长度为 arrLen 的数组，开始有一个指针在索引 0 处。

每一步操作中，你可以将指针向左或向右移动 1 步，或者停在原地（指针不能被移动到数组范围外）。

给你两个整数 steps 和 arrLen ，请你计算并返回：在恰好执行 steps 次操作以后，指针仍然指向索引 0 处的方案数。

由于答案可能会很大，请返回方案数 模 10^9 + 7 后的结果。

https://leetcode.cn/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/description/
*/
pub struct Solution;

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let steps = steps as usize;
        let arr_len = (steps + 1).min(arr_len as usize);
        let mut dp: Vec<Vec<i64>> = vec![vec![0; arr_len]; steps + 1];
        dp[0][0] = 1;

        for i in 1..=steps {
            for j in 0..(i + 1).min(arr_len) {
                dp[i][j] += dp[i - 1][j];

                if j >= 1 {
                    dp[i][j] += dp[i - 1][j - 1];
                }
                if j < arr_len - 1 {
                    dp[i][j] += dp[i - 1][j + 1];
                }

                dp[i][j] %= Self::VAL_MOD;
            }
        }

        dp[steps][0] as i32
    }
}

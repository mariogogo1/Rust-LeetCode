/**

100292. 找出所有稳定的二进制数组 I
100293. 找出所有稳定的二进制数组 II

给你 3 个正整数 zero ，one 和 limit 。

一个
二进制数组
 arr 如果满足以下条件，那么我们称它是 稳定的 ：

0 在 arr 中出现次数 恰好 为 zero 。
1 在 arr 中出现次数 恰好 为 one 。
arr 中每个长度超过 limit 的
子数组
 都 同时 包含 0 和 1 。
请你返回 稳定 二进制数组的 总 数目。

由于答案可能很大，将它对 109 + 7 取余 后返回。

https://leetcode.cn/problems/find-all-possible-stable-binary-arrays-i/description/
https://leetcode.cn/problems/find-all-possible-stable-binary-arrays-ii/description/
*/
pub struct Solution;

/// dp_0[i][j] 為使用了i個0跟j個1且結尾為0的總數
/// dp_1[i][j] 為使用了i個0跟j個1且結尾為1的總數
/// 根據條件 1數字結尾的數字 可以接續[1,limit]個0，不能連續接超過limit個0
/// 轉移方程組  dp_0[i][j] = dp_1[i-1][j] + dp_1[i-2][j] + .... + dp_1[i-limit][j]
/// 可以使用前綴和改寫 dp_0[i][j] = prefix_sum_1[i-1][j] - prefix_sum_1[i-limit-1][j]
///  prefix_sum_1[i][j] = prefix_sum_1[i-1][j] + dp_1[i][j];
///  邊界條件  dp_0[0][0] =1;
///  prefix_sum_0[0][0] = 1;

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let zero = zero as usize;
        let one = one as usize;
        let limit = limit as usize;
        let mut dp_0: Vec<Vec<i64>> = vec![vec![0; one + 1]; zero + 1];
        let mut dp_1: Vec<Vec<i64>> = vec![vec![0; one + 1]; zero + 1];
        let mut prefix_sum_0: Vec<Vec<i64>> = vec![vec![0; one + 1]; zero + 1];
        let mut prefix_sum_1: Vec<Vec<i64>> = vec![vec![0; one + 1]; zero + 1];

        dp_0[0][0] = 1;
        dp_1[0][0] = 1;
        prefix_sum_0[0][0] = 1;
        prefix_sum_1[0][0] = 1;

        for i in 0..=zero {
            for j in 0..=one {
                if i > 0 {
                    dp_0[i][j] += prefix_sum_1[i - 1][j];
                }
                if i > limit {
                    dp_0[i][j] -= prefix_sum_1[i - limit - 1][j];
                }
                if j > 0 {
                    dp_1[i][j] += prefix_sum_0[i][j - 1];
                }
                if j > limit {
                    dp_1[i][j] -= prefix_sum_0[i][j - limit - 1];
                }
                if i > 0 {
                    prefix_sum_1[i][j] = prefix_sum_1[i - 1][j] + dp_1[i][j];
                } else {
                    prefix_sum_1[i][j] = dp_1[i][j];
                }
                if j > 0 {
                    prefix_sum_0[i][j] = prefix_sum_0[i][j - 1] + dp_0[i][j];
                } else {
                    prefix_sum_0[i][j] = dp_0[i][j];
                }
                dp_0[i][j] %= Self::VAL_MOD;
                dp_1[i][j] %= Self::VAL_MOD;
                prefix_sum_0[i][j] %= Self::VAL_MOD;
                prefix_sum_1[i][j] %= Self::VAL_MOD;
            }
        }

        let mut ans = ((dp_0[zero][one] + dp_1[zero][one]) % Self::VAL_MOD);
        if ans < 0 {
            ans += Self::VAL_MOD;
        }

        ans as i32
    }
}

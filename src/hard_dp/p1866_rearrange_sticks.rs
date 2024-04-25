/**

1866. 恰有 K 根木棍可以看到的排列数目

有 n 根长度互不相同的木棍，长度为从 1 到 n 的整数。请你将这些木棍排成一排，并满足从左侧 可以看到 恰好 k 根木棍。从左侧 可以看到 木棍的前提是这个木棍的 左侧 不存在比它 更长的 木棍。

例如，如果木棍排列为 [1,3,2,5,4] ，那么从左侧可以看到的就是长度分别为 1、3 、5 的木棍。
给你 n 和 k ，返回符合题目要求的排列 数目 。由于答案可能很大，请返回对 109 + 7 取余 的结果。

提示：

1 <= n <= 1000
1 <= k <= n

https://leetcode.cn/problems/number-of-ways-to-rearrange-sticks-with-k-sticks-visible/description/
*/
pub struct Solution;

/// dp[i][j] 紀錄為看起來有i跟但實際上有j根的種類數量
/// 從最長的那一根開始操作，由長而短，此方法每次放入的時候都是當前最短的一根，
/// 假設當前柱子總共是X根，放在所有的柱子前面就會使"可以看見的長度"增加1，放在其餘位置總共有X個位置可以選擇，但長度不變
///
/// dp[i][j] = 1*dp[i-1][j-1] + (j-1)*dp[i][j-1]

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut dp: Vec<Vec<i64>> = vec![vec![0; n + 1]; k + 1];
        dp[0][0] = 1;
        for j in 1..=n {
            for i in 1..=k {
                dp[i][j] = dp[i - 1][j - 1] + (j - 1) as i64 * dp[i][j - 1];
                dp[i][j] %= Self::VAL_MOD;
            }
        }
        dp[k][n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::rearrange_sticks(3, 2), 3);
        assert_eq!(Solution::rearrange_sticks(5, 5), 1);
        assert_eq!(Solution::rearrange_sticks(20, 11), 647427950);
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

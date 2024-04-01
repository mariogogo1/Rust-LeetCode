/**

3082. 求出所有子序列的能量和

给你一个长度为 n 的整数数组 nums 和一个 正 整数 k 。

一个整数数组的 能量 定义为和 等于 k 的子序列的数目。

请你返回 nums 中所有子序列的 能量和 。

由于答案可能很大，请你将它对 109 + 7 取余 后返回。

提示：

1 <= n <= 100
1 <= nums[i] <= 104
1 <= k <= 100

https://leetcode.cn/problems/find-the-sum-of-the-power-of-all-subsequences/description/
*/
pub struct Solution;

/// dp[k] 紀錄能量為K的總數量
/// 當數組由[a,b,c]增加到[a,b,c,d]時
/// 所有的dp[k]由[a,b,c]組成的各種結果 會增加一倍，因為所有組合都多了一個d加入的子序列
///             []   [a]    [b]     [c]  .... [a,b,c]
/// 加入d之後    [d]  [a,d]   [b,d]    [c,d] ....[a,b,c,d]
/// 多出了一倍數量
/// 然後再加上dp[k-num]的數量
/// 所以反向倒敘 新dp[k] = 舊dp[k]*2 +dp[k-num] 可以更新dp
/// 把所有數num遍歷更新完畢 dp[k] 即為解

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    // O(NK)
    pub fn sum_of_power(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k_usize = k as usize;
        let mut dp: Vec<i64> = vec![0; k_usize + 1];
        dp[0] = 1;
        for num in nums {
            for i in (0..=k_usize).rev() {
                dp[i] += dp[i];
                if i as i32 - num >= 0 {
                    dp[i] += dp[i - num as usize];
                }
                dp[i] %= Self::VAL_MOD;
            }
        }

        dp[k_usize] as i32
    }

    fn my_pow(x: i64, n: i32) -> i64 {
        fn quick_pow(x: i64, n: i64) -> i64 {
            if n == 0 {
                return 1;
            }
            let a = quick_pow(x, n / 2);
            if n % 2 == 0 {
                return (a * a) % Solution::VAL_MOD;
            }
            return (a * a * x) % Solution::VAL_MOD;
        }

        return quick_pow(x, n as i64);
    }
    /// 參數沒有很大，直接硬做  O(N*N*K)
    pub fn sum_of_power_basic(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k_usize = k as usize;
        let mut dp: Vec<Vec<i64>> = vec![vec![0; n + 1]; k_usize + 1];
        dp[0][0] = 1;
        for num in nums {
            for i in (0..=k_usize).rev() {
                for j in (1..=n).rev() {
                    if i as i32 - num >= 0 {
                        dp[i][j] += dp[i - num as usize][j - 1];
                        dp[i][j] %= Self::VAL_MOD
                    }
                }
            }
        }
        let mut ans: i64 = 0;
        for i in 1..=n {
            ans += (dp[k_usize][i] * Self::my_pow(2, (n - i) as i32)) % Self::VAL_MOD;
            ans %= Self::VAL_MOD;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::sum_of_power(vec![1, 2, 3], 3), 6);
        assert_eq!(
            Solution::sum_of_power(
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                ],
                25
            ),
            723214743
        );
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

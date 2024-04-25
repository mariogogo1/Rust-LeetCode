/**
1223. 掷骰子模拟

有一个骰子模拟器会每次投掷的时候生成一个 1 到 6 的随机数。

不过我们在使用它时有个约束，就是使得投掷骰子时，连续 掷出数字 i 的次数不能超过 rollMax[i]（i 从 1 开始编号）。

现在，给你一个整数数组 rollMax 和一个整数 n，请你来计算掷 n 次骰子可得到的不同点数序列的数量。

假如两个序列中至少存在一个元素不同，就认为这两个序列是不同的。由于答案可能很大，所以请返回 模 10^9 + 7 之后的结果。

提示：

1 <= n <= 5000
rollMax.length == 6
1 <= rollMax[i] <= 15

https://leetcode.cn/problems/dice-roll-simulation/description/
*/
pub struct Solution;
/// dp[i][j] i 為骰子點數，j為記錄序列長度，dp[i][j] = 長度為j最後一個骰子點數為i的所有可能
///
///
/// dp[i][j] = sum(dp[x][j-s]) , x =1~6 && x != i &&  min(j, limit) >= s >= 1
/// dp[i][0] = 1 但是實作上可以調整，已利用和的快速計算         
///
/// 核心想法：
/// 假設關鍵字元是a limit是3
/// 那麼dp[a][5]的字串 = (dp[其他][4]的字串+"a") + (dp[其他][3]的字串+"aa") + (dp[其他][2]的字串+"aaa")
///                         yyyxa                      yyxaa                     yxaaa  
/// x,y =1~6 && x != a
///

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let mut max_extra: i32 = 0;
        for &i in roll_max.iter() {
            max_extra = max_extra.max(i);
        }
        let m = max_extra as usize;
        let n_usize = n as usize;
        let mut dp: Vec<Vec<i64>> = vec![vec![0; n_usize + m + 1]; 7];
        dp[6][m] = 1;
        let mut sum;
        for j in m + 1..dp[0].len() {
            sum = 0;
            for i in 0..6 {
                dp[i][j] = dp[6][j - 1] - dp[6][j - 1 - roll_max[i] as usize]
                    + dp[i][j - 1 - roll_max[i] as usize];
                sum += dp[i][j];
            }
            dp[6][j] = sum % Self::VAL_MOD;
        }
        // 避免出現負數
        if dp[6][n_usize + m] < 0 {
            dp[6][n_usize + m] += Self::VAL_MOD;
        }
        return dp[6][n_usize + m] as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::die_simulator(2, vec![1, 1, 2, 2, 2, 3]), 34);
        assert_eq!(Solution::die_simulator(2, vec![1, 1, 1, 1, 1, 1]), 30);
        assert_eq!(Solution::die_simulator(3, vec![1, 1, 1, 2, 2, 3]), 181);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::die_simulator(5, vec![1, 1, 1, 2, 2, 3]), 5428);
        assert_eq!(Solution::die_simulator(8, vec![2, 7, 1, 2, 6, 5]), 1336363);
    }
}

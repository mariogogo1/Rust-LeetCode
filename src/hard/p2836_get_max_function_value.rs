/**

2836. 在传球游戏中最大化函数值

给你一个长度为 n 下标从 0 开始的整数数组 receiver 和一个整数 k 。

总共有 n 名玩家，玩家 编号 互不相同，且为 [0, n - 1] 中的整数。这些玩家玩一个传球游戏，receiver[i] 表示编号为 i 的玩家会传球给编号为 receiver[i] 的玩家。玩家可以传球给自己，也就是说 receiver[i] 可能等于 i 。

你需要从 n 名玩家中选择一名玩家作为游戏开始时唯一手中有球的玩家，球会被传 恰好 k 次。

如果选择编号为 x 的玩家作为开始玩家，定义函数 f(x) 表示从编号为 x 的玩家开始，k 次传球内所有接触过球玩家的编号之 和 ，如果有玩家多次触球，则 累加多次 。换句话说， f(x) = x + receiver[x] + receiver[receiver[x]] + ... + receiver(k)[x] 。

你的任务时选择开始玩家 x ，目的是 最大化 f(x) 。

请你返回函数的 最大值 。

注意：receiver 可能含有重复元素。

提示：

1 <= receiver.length == n <= 105
0 <= receiver[i] <= n - 1
1 <= k <= 1010

https://leetcode.cn/problems/maximize-value-of-function-in-a-ball-passing-game/description/
*/

struct Solution {}

/// K給出的範圍是10^9非常的大，需要使用位元運算
/// 可以計算出 從任意一個點出發 傳球2^i次的總和以及傳到的對象終點，i = [0,log2(K)]
/// dp[i][j] 紀錄為從j對象傳了2^i次的總和
/// get[i][j] 紀錄為從j對象傳了2^i次的後傳到的對象
/// dp[i+1][j] = dp[i][j] + dp[i][get[i][j]]
/// 最後計算每個人開始的F(X)取MAX
/// 時間複雜度O(N*log2(K))

impl Solution {
    fn floor_log_2(k: i64) -> usize {
        let k = k as u64;
        (64 - k.leading_zeros()) as usize
    }

    pub fn get_max_function_value(receiver: Vec<i32>, mut k: i64) -> i64 {
        let n_usize = receiver.len();
        let k_usize = Self::floor_log_2(k);
        let mut receiver_i64: Vec<i64> = vec![0; n_usize]; // 初始值
        let mut now_index: Vec<usize> = vec![0; n_usize]; // 初始站的位置
        let mut dp: Vec<Vec<i64>> = vec![vec![0; n_usize]; k_usize];
        let mut get: Vec<Vec<usize>> = vec![vec![0; n_usize]; k_usize];
        for i in 0..n_usize {
            // 第一次傳球
            dp[0][i] = receiver[i] as i64;
            get[0][i] = receiver[i] as usize;
            now_index[i] = i;
            receiver_i64[i] = i as i64;
        }

        for i in 1..k_usize {
            for j in 0..n_usize {
                dp[i][j] = dp[i - 1][j] + dp[i - 1][get[i - 1][j]];
                get[i][j] = get[i - 1][get[i - 1][j]];
            }
        }
        //計算從每個球員開始當起點傳了K次的結果
        let mut ans: i64 = 0;

        for i in 0..k_usize {
            if k & 1 == 1 {
                for j in 0..n_usize {
                    receiver_i64[j] += dp[i][now_index[j]] as i64;
                    now_index[j] = get[i][now_index[j]];
                }
            }
            k = k >> 1;
        }

        for i in 0..n_usize {
            ans = ans.max(receiver_i64[i]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::get_max_function_value(vec![2, 0, 1], 4), 6);
        assert_eq!(Solution::get_max_function_value(vec![1, 1, 1, 2, 3], 3), 10);
    }
    #[test]
    fn test_case_1() {
        assert_eq!(Solution::get_max_function_value(vec![1, 2, 0, 0], 14), 16);
    }
    #[test]
    fn test_case_2() {}
}

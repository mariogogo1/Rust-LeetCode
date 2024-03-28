/**
650. 两个键的键盘

最初记事本上只有一个字符 'A' 。你每次可以对这个记事本进行两种操作：

Copy All（复制全部）：复制这个记事本中的所有字符（不允许仅复制部分字符）。
Paste（粘贴）：粘贴 上一次 复制的字符。
给你一个数字 n ，你需要使用最少的操作次数，在记事本上输出 恰好 n 个 'A' 。返回能够打印出 n 个 'A' 的最少操作次数。

提示：

1 <= n <= 1000

https://leetcode.cn/problems/2-keys-keyboard/description/
*/
pub struct Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n_usize = n as usize;
        let mut ops_dp: Vec<i32> = vec![n; n_usize + 1];
        ops_dp[0] = 0;
        ops_dp[1] = 0;
        // let n_sqrt = (n as f64 + 0.05).sqrt() as usize + 1;
        for i in 1..n_usize {
            if n_usize % i == 0 {
                for j in 2..=(n_usize / i) {
                    ops_dp[j * i] = ops_dp[j * i].min(ops_dp[i] + j as i32);
                }
            }
        }
        //println!("{:?}", ops_dp);
        return ops_dp[n_usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::min_steps(12), 7);
        assert_eq!(Solution::min_steps(11), 11);
        assert_eq!(Solution::min_steps(10), 7);
        assert_eq!(Solution::min_steps(1), 0);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::min_steps(18), 8);
    }
}

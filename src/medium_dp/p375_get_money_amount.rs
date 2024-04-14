/**
375. 猜数字大小 II

我们正在玩一个猜数游戏，游戏规则如下：

我从 1 到 n 之间选择一个数字。
你来猜我选了哪个数字。
如果你猜到正确的数字，就会 赢得游戏 。
如果你猜错了，那么我会告诉你，我选的数字比你的 更大或者更小 ，并且你需要继续猜数。
每当你猜了数字 x 并且猜错了的时候，你需要支付金额为 x 的现金。如果你花光了钱，就会 输掉游戏 。
给你一个特定的数字 n ，返回能够 确保你获胜 的最小现金数，不管我选择那个数字 。

https://leetcode.cn/problems/guess-number-higher-or-lower-ii/description/
*/
/// dp[i][j] 從J樓往上到I樓的最小花費
impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        if n == 2 {
            return 1;
        }
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; n + 1];

        dp[2][1] = 1;
        for i in 3..=n {
            dp[i][i - 1] = (i - 1) as i32;
            dp[i][i - 2] = (i - 1) as i32;
        }

        for i in 4..=n {
            for j in (1..=(i - 3)).rev() {
                dp[i][j] = i32::MAX;
                for k in j..i {
                    let x = dp[k - 1][j].max(dp[i][k + 1]) + k as i32;
                    dp[i][j] = dp[i][j].min(x);
                }
            }
        }

        dp[n][1]
    }
}

/**
799. 香槟塔

我们把玻璃杯摆成金字塔的形状，其中 第一层 有 1 个玻璃杯， 第二层 有 2 个，依次类推到第 100 层，每个玻璃杯 (250ml) 将盛有香槟。

从顶层的第一个玻璃杯开始倾倒一些香槟，当顶层的杯子满了，任何溢出的香槟都会立刻等流量的流向左右两侧的玻璃杯。当左右两边的杯子也满了，就会等流量的流向它们左右两边的杯子，依次类推。（当最底层的玻璃杯满了，香槟会流到地板上）

例如，在倾倒一杯香槟后，最顶层的玻璃杯满了。倾倒了两杯香槟后，第二层的两个玻璃杯各自盛放一半的香槟。在倒三杯香槟后，第二层的香槟满了 - 此时总共有三个满的玻璃杯。在倒第四杯后，第三层中间的玻璃杯盛放了一半的香槟，他两边的玻璃杯各自盛放了四分之一的香槟，如下图所示。

https://leetcode.cn/problems/champagne-tower/description/
*/
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let query_row = query_row as usize;
        let mut dp: Vec<Vec<f64>> = vec![vec![0.0; query_row + 1]; query_row + 1];
        dp[0][0] = poured as f64;

        for i in 0..query_row {
            for j in 0..=i {
                if dp[i][j] > 1.0 {
                    dp[i + 1][j] += (dp[i][j] - 1.0) / 2.0;
                    dp[i + 1][j + 1] += (dp[i][j] - 1.0) / 2.0;
                }
            }
        }

        dp[query_row as usize][query_glass as usize].min(1.0)
    }
}

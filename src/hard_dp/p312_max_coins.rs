/**
312. 戳气球

有 n 个气球，编号为0 到 n - 1，每个气球上都标有一个数字，这些数字存在数组 nums 中。

现在要求你戳破所有的气球。戳破第 i 个气球，你可以获得 nums[i - 1] * nums[i] * nums[i + 1] 枚硬币。 这里的 i - 1 和 i + 1 代表和 i 相邻的两个气球的序号。如果 i - 1或 i + 1 超出了数组的边界，那么就当它是一个数字为 1 的气球。

求所能获得硬币的最大数量。

https://leetcode.cn/problems/burst-balloons/description/
*/
pub struct Solution;
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = vec![vec![0; n + 2]; n + 2];
        let mut val = vec![0; n + 2];
        val[0] = 1;
        val[n + 1] = 1;
        for i in 1..=n {
            val[i] = nums[i - 1];
        }
        for i in (0..n).rev() {
            for j in i + 2..=n + 1 {
                for k in i + 1..j {
                    ans[i][j] = ans[i][j].max(val[i] * val[k] * val[j] + ans[i][k] + ans[k][j]);
                }
            }
        }
        ans[0][n + 1]
    }
}

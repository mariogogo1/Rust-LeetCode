/**

1411. 给 N x 3 网格图涂色的方案数

你有一个 n x 3 的网格图 grid ，你需要用 红，黄，绿 三种颜色之一给每一个格子上色，且确保相邻格子颜色不同（也就是有相同水平边或者垂直边的格子颜色不同）。

给你网格图的行数 n 。

请你返回给 grid 涂色的方案数。由于答案可能会非常大，请你返回答案对 10^9 + 7 取余的结果。

https://leetcode.cn/problems/number-of-ways-to-paint-n-3-grid/description/
*/
pub struct Solution;
/// 通用解法可以看1931
/// 1411的寬度只有3，所以可以討論
/// 分類一：aba
/// 分類二：abc
/// 每個分類剛好都有6種情況，
/// dp_1[i] 代表類型一為第i排時的所有情況  
/// dp_2[i] 代表類型二為第i排時的所有情況  
/// dp_1[i] = 3*dp_1[i-1] + 2*dp_2[i-1]  
/// dp_2[i] = 2*dp_1[i-1] + 2*dp_2[i-1]  
///

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn num_of_ways(n: i32) -> i32 {
        let n = n as usize;

        let mut dp_1 = vec![0; n];
        let mut dp_2 = vec![0; n];

        dp_1[0] = 6;
        dp_2[0] = 6;

        for i in 1..n {
            dp_1[i] = 3 * dp_1[i - 1] + 2 * dp_2[i - 1];
            dp_2[i] = 2 * dp_1[i - 1] + 2 * dp_2[i - 1];
            dp_1[i] %= Self::VAL_MOD;
            dp_2[i] %= Self::VAL_MOD;
        }

        ((dp_1[n - 1] + dp_2[n - 1]) % Self::VAL_MOD) as i32
    }
}

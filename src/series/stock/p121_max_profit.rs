/**

121. 买卖股票的最佳时机

给定一个数组 prices ，它的第 i 个元素 prices[i] 表示一支给定股票第 i 天的价格。

你只能选择 某一天 买入这只股票，并选择在 未来的某一个不同的日子 卖出该股票。设计一个算法来计算你所能获取的最大利润。

返回你可以从这笔交易中获取的最大利润。如果你不能获取任何利润，返回 0 。


https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/description/
*/
pub struct Solution;

/// 動態規劃經典系列題組
/// 相關：
/// 121. 买卖股票的最佳时机
/// 122. 买卖股票的最佳时机 II
/// 123. 买卖股票的最佳时机 III
/// 188. 买卖股票的最佳时机 IV
/// 309. 买卖股票的最佳时机含冷冻期
/// 714. 买卖股票的最佳时机含手续费
///
/// 核心想法：
/// 製作兩個動態規劃狀態
/// dp_hold[k][j] 持有股票 交易k次 第j天的最大營利
/// dp_nonehold[k][j] 未持有股票 交易k次 第j天的最大營利
/// dp_hold[k][0] = -inf
/// dp_nonehold[k][0] = 0
///
/// dp_hold[k][j] = max(dp_nonehold[k-1][j-1]-prices[j],dp_hold[k][j-1])
/// dp_nonehold[k][j] = max(dp_hold[k][j-1]+prices[j],dp_nonhold[k][j-1])
///
/// 最後日期的營利 一定是，未持有股票(把股票賣了) > 持有股票， 所以最後判斷 dp_nonehold 即可知道最大營利
///
/// 122. 买卖股票的最佳时机 II 可交易無限次，而且知道所有的價格->只要隔天-今天的營利>0，就納入答案。
/// 以動態規劃來說 K次交易沒有限制就不用紀錄K的次數，作法同可參考309. 买卖股票的最佳时机含冷冻期 & 714. 买卖股票的最佳时机含手续费
/// 有手續費fee跟冷凍期frozen的做以下調整
/// dp_nonehold[k][j] = max(dp_hold[k][j-1]+prices[j-forzen],dp_nonhold[k][j-1])
/// dp_hold[k][j] = max(dp_nonehold[k-1][j-1-forzen]-prices[j-forzen]-fee,dp_hold[k][j-1])
///
/// 121. 买卖股票的最佳时机 簡單題如果要用動態規劃肯定比較慢，只交易一次可以改成紀錄當前價格的min,每天更新max(price[j]-min)即可減低空間複雜度。
///

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let k = 1 as usize;
        let mut dp_hold: Vec<Vec<i32>> = vec![vec![i32::MIN; n + 1]; k + 1];
        let mut dp_none_hold: Vec<Vec<i32>> = vec![vec![0; n + 1]; k + 1];
        let mut ans: i32 = 0;
        for i in 1..=k {
            for j in 1..=n {
                dp_hold[i][j] = dp_hold[i][j - 1].max(dp_none_hold[i - 1][j - 1] - prices[j - 1]);
                dp_none_hold[i][j] = dp_none_hold[i][j - 1].max(dp_hold[i][j - 1] + prices[j - 1]);
            }
        }
        //println!("{:?},{:?}", dp_hold, dp_none_hold);
        for i in 0..=k {
            ans = ans.max(dp_none_hold[i][n]);
        }
        ans
    }
}

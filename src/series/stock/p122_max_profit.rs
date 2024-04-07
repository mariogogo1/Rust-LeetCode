/**

122. 买卖股票的最佳时机 II

给你一个整数数组 prices ，其中 prices[i] 表示某支股票第 i 天的价格。

在每一天，你可以决定是否购买和/或出售股票。你在任何时候 最多 只能持有 一股 股票。你也可以先购买，然后在 同一天 出售。

返回 你能获得的 最大 利润 。


提示：

1 <= prices.length <= 3 * 104
0 <= prices[i] <= 104

https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii/description/
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
        let mut ans = 0;
        for i in 1..prices.len() {
            if prices[i] - prices[i - 1] > 0 {
                ans += prices[i] - prices[i - 1];
            }
        }
        ans
    }
}

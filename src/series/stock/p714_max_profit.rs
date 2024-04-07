/**

714. 买卖股票的最佳时机含手续费

给定一个整数数组 prices，其中 prices[i]表示第 i 天的股票价格 ；整数 fee 代表了交易股票的手续费用。

你可以无限次地完成交易，但是你每笔交易都需要付手续费。如果你已经购买了一个股票，在卖出它之前你就不能再继续购买股票了。

返回获得利润的最大值。

注意：这里的一笔交易指买入持有并卖出股票的整个过程，每笔交易你只需要为支付一次手续费。


提示：

1 <= prices.length <= 5 * 104
1 <= prices[i] < 5 * 104
0 <= fee < 5 * 104

https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/description/
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
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();
        let mut dp_hold: Vec<i32> = vec![i32::MIN; n + 1];
        let mut dp_none_hold: Vec<i32> = vec![0; n + 1];
        let mut ans: i32 = 0;

        for j in 1..=n {
            dp_hold[j] = dp_hold[j - 1].max(dp_none_hold[j - 1] - prices[j - 1] - fee);
            dp_none_hold[j] = dp_none_hold[j - 1].max(dp_hold[j - 1] + prices[j - 1]);
        }
        // println!("{:?},{:?}", dp_hold, dp_none_hold);

        ans = ans.max(dp_none_hold[n]);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

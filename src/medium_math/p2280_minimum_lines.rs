/**
2280. 表示一个折线图的最少线段数

给你一个二维整数数组 stockPrices ，其中 stockPrices[i] = [dayi, pricei] 表示股票在 dayi 的价格为 pricei 。折线图 是一个二维平面上的若干个点组成的图，横坐标表示日期，纵坐标表示价格，折线图由相邻的点连接而成。比方说下图是一个例子：

https://leetcode.cn/problems/minimum-lines-to-represent-a-line-chart/description/
*/

pub struct Solution;

impl Solution {
    pub fn minimum_lines(mut stock_prices: Vec<Vec<i32>>) -> i32 {
        stock_prices.sort_by(|x, y| x[0].cmp(&y[0]));
        let mut slope_x: i64 = 0;
        let mut slope_y: i64 = i32::MAX as i64;
        let mut ans = 0;
        for i in 1..stock_prices.len() {
            let delta_x = stock_prices[i][0] - stock_prices[i - 1][0];
            let delta_y = stock_prices[i][1] - stock_prices[i - 1][1];

            if (slope_x * delta_y as i64) != (slope_y * delta_x as i64) {
                ans += 1;
                slope_x = delta_x as i64;
                slope_y = delta_y as i64;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::minimum_lines(vec![
                vec![1, 7],
                vec![2, 6],
                vec![3, 5],
                vec![4, 4],
                vec![5, 4],
                vec![6, 3],
                vec![7, 2],
                vec![8, 1]
            ]),
            3
        );
    }
    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::minimum_lines(vec![vec![3, 4], vec![1, 2], vec![7, 8], vec![2, 3]]),
            1
        );
    }
    #[test]
    fn test_case_2() {}
}

/**
901. 股票价格跨度

设计一个算法收集某些股票的每日报价，并返回该股票当日价格的 跨度 。

当日股票价格的 跨度 被定义为股票价格小于或等于今天价格的最大连续日数（从今天开始往回数，包括今天）。

例如，如果未来 7 天股票的价格是 [100,80,60,70,60,75,85]，那么股票跨度将是 [1,1,1,2,1,4,6] 。

实现 StockSpanner 类：

StockSpanner() 初始化类对象。
int next(int price) 给出今天的股价 price ，返回该股票当日价格的 跨度 。

https://leetcode.cn/problems/online-stock-span/description/
*/
struct StockSpanner {
    prices: Vec<i32>,
    stack: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        StockSpanner {
            prices: vec![i32::MAX],
            stack: vec![0],
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        while let Some(&top) = self.stack.last() {
            if self.prices[top] <= price {
                self.stack.pop();
            } else {
                break;
            }
        }
        self.stack.push(self.prices.len());
        self.prices.push(price);
        let n = self.stack.len();
        (self.stack[n - 1] - self.stack[n - 2]) as i32
    }
}

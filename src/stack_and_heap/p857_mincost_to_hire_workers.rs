/**
857. 雇佣 K 名工人的最低成本

有 n 名工人。 给定两个数组 quality 和 wage ，其中，quality[i] 表示第 i 名工人的工作质量，其最低期望工资为 wage[i] 。

现在我们想雇佣 k 名工人组成一个工资组。在雇佣 一组 k 名工人时，我们必须按照下述规则向他们支付工资：

对工资组中的每名工人，应当按其工作质量与同组其他工人的工作质量的比例来支付工资。
工资组中的每名工人至少应当得到他们的最低期望工资。
给定整数 k ，返回 组成满足上述条件的付费群体所需的最小金额 。在实际答案的 10-5 以内的答案将被接受。。

https://leetcode.cn/problems/minimum-cost-to-hire-k-workers/description/
*/
pub struct Solution;
use std::collections::BinaryHeap;

/// 对工资组中的每名工人，应当按其工作质量与同组其他工人的工作质量的比例来支付工资：這句話的意思每個員工都希望自己每單位的quality可以換成同等價值的wage
/// 舉例：A員工 輸出每單位quality 可以換 2單位的wage
/// 後面又進來了一個B員工  輸出每單位quality 可以換 3單位的wage
/// 這時A員工就會要求加薪至少要跟B員工一樣，A輸出每單位quality 也可以換 3單位的wage
/// 這樣就清楚可以把總薪資寫成
/// total = max(wage[i]/quality[i]) * sum(quality[i])
/// 可以想成時薪最高的單價*所有人的總工時
///
/// 實作：將wage[i]/quality[i] 做遞增排序
/// 維護一個大根堆計算(K-1)個人的quality總和，挑出(K-1)個最小的quality

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let n = quality.len();
        let k = k as usize;
        let mut pair: Vec<(f64, i32)> = vec![];
        for i in 0..n {
            pair.push((wage[i] as f64 / quality[i] as f64, quality[i]));
        }
        pair.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap()); // Sort by wage/quality ratio

        let mut ans: f64 = f64::MAX;
        let mut min_heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut total_quality = 0;
        for i in 0..(k - 1) {
            min_heap.push(pair[i].1);
            total_quality += pair[i].1;
        }

        for i in (k - 1)..n {
            total_quality += pair[i].1;
            ans = ans.min(total_quality as f64 * pair[i].0);
            total_quality -= pair[i].1;

            // pair[i].1維護進大根堆裡面
            if let Some(&val) = min_heap.peek() {
                if val > pair[i].1 {
                    total_quality -= val;
                    total_quality += pair[i].1;
                    min_heap.pop();
                    min_heap.push(pair[i].1);
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case() {}
}

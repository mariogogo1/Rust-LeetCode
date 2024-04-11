/**
502. IPO

假设 力扣（LeetCode）即将开始 IPO 。为了以更高的价格将股票卖给风险投资公司，力扣 希望在 IPO 之前开展一些项目以增加其资本。 由于资源有限，它只能在 IPO 之前完成最多 k 个不同的项目。帮助 力扣 设计完成最多 k 个不同项目后得到最大总资本的方式。

给你 n 个项目。对于每个项目 i ，它都有一个纯利润 profits[i] ，和启动该项目需要的最小资本 capital[i] 。

最初，你的资本为 w 。当你完成一个项目时，你将获得纯利润，且利润将被添加到你的总资本中。

总而言之，从给定项目中选择 最多 k 个不同项目的列表，以 最大化最终资本 ，并输出最终可获得的最多资本。

答案保证在 32 位有符号整数范围内。

https://leetcode.cn/problems/ipo/description/
*/

pub struct Solution;
use std::collections::BinaryHeap;

// 排序後用大根推挑選報酬最高的項目 重複 k 次
impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut pair: Vec<(&i32, &i32)> = capital.iter().zip(profits.iter()).collect();
        pair.sort_by(|i, j| j.0.cmp(&i.0)); // sort_unstable_by 會快超級多!!!!
        let mut max_heap = BinaryHeap::new();

        for i in 0..k {
            while let Some(&the_pair) = pair.last() {
                if *the_pair.0 <= w {
                    max_heap.push(*the_pair.1);
                    pair.pop();
                } else {
                    break;
                }
            }
            if let Some(top) = max_heap.pop() {
                w += top;
            } else {
                break;
            }
        }
        w
    }
}

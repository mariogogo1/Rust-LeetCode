/**
826. 安排工作以达到最大收益

你有 n 个工作和 m 个工人。给定三个数组： difficulty, profit 和 worker ，其中:

difficulty[i] 表示第 i 个工作的难度，profit[i] 表示第 i 个工作的收益。
worker[i] 是第 i 个工人的能力，即该工人只能完成难度小于等于 worker[i] 的工作。
每个工人 最多 只能安排 一个 工作，但是一个工作可以 完成多次 。

举个例子，如果 3 个工人都尝试完成一份报酬为 $1 的同样工作，那么总收益为 $3 。如果一个工人不能完成任何工作，他的收益为 $0 。
返回 在把工人分配到工作岗位后，我们所能获得的最大利润 。
https://leetcode.cn/problems/most-profit-assigning-work/description/
*/
pub struct Solution;
impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut workers: Vec<i32>,
    ) -> i32 {
        let mut pairs: Vec<(i32, i32)> = difficulty.into_iter().zip(profit).collect();
        let n = workers.len();

        pairs.sort_unstable();
        workers.sort_unstable();

        let mut ans = 0;
        let mut max_v = 0;
        let mut idx = 0 as usize;

        for worker in workers {
            while idx < n && worker >= pairs[idx].0 {
                max_v = max_v.max(pairs[idx].1);

                idx += 1;
            }
            ans += max_v;
        }

        ans
    }
}

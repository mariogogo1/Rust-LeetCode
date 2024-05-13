/**
1235. 规划兼职工作

你打算利用空闲时间来做兼职工作赚些零花钱。

这里有 n 份兼职工作，每份工作预计从 startTime[i] 开始到 endTime[i] 结束，报酬为 profit[i]。

给你一份兼职工作表，包含开始时间 startTime，结束时间 endTime 和预计报酬 profit 三个数组，请你计算并返回可以获得的最大报酬。

注意，时间上出现重叠的 2 份工作不能同时进行。

如果你选择的工作在时间 X 结束，那么你可以立刻进行在时间 X 开始的下一份工作。

https://leetcode.cn/problems/maximum-profit-in-job-scheduling/description
*/
pub struct Solution;
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut three_pairs: Vec<(i32, i32, i32)> = start_time
            .iter()
            .zip(end_time.iter())
            .zip(profit.iter()) // key 1
            .map(|((&x, &y), &z)| (x, y, z)) // key 2
            .collect();

        three_pairs.sort_unstable_by_key(|&(_, y, _)| y); // key 3
        let mut dp = vec![0; n + 1];

        for i in 1..=n {
            let pos = three_pairs[..i].partition_point(|the_obj| the_obj.1 <= three_pairs[i - 1].0); // key 4 找到切片中第一個符合閉包條件的元素下標
            dp[i] = dp[i - 1].max(three_pairs[i - 1].2 + dp[pos]);
        }

        dp[n]
    }
}

/**
2589. 完成所有任务的最少时间

你有一台电脑，它可以 同时 运行无数个任务。给你一个二维整数数组 tasks ，其中 tasks[i] = [starti, endi, durationi] 表示第 i 个任务需要在 闭区间 时间段 [starti, endi] 内运行 durationi 个整数时间点（但不需要连续）。

当电脑需要运行任务时，你可以打开电脑，如果空闲时，你可以将电脑关闭。

请你返回完成所有任务的情况下，电脑最少需要运行多少秒。

https://leetcode.cn/problems/minimum-time-to-complete-all-tasks/description/
*/
pub struct Solution;
impl Solution {
    pub fn find_minimum_time(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let n = tasks.len();
        let mut run_time: Vec<bool> = vec![false; tasks[n - 1][1] as usize + 1];

        let mut ans = 0;
        for task in tasks {
            let s = task[0] as usize;
            let mut e = task[1] as usize;
            let mut d = task[2] as usize;
            for i in s..=e {
                if run_time[i] {
                    d -= 1;
                }
            }

            if d == 0 || d > e {
                continue;
            }

            while d > 0 {
                if !run_time[e] {
                    run_time[e] = true;
                    d -= 1;
                    ans += 1;
                }
                e -= 1;
            }
        }
        ans
    }
}

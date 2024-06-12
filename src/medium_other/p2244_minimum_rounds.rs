/**
2244. 完成所有任务需要的最少轮数

给你一个下标从 0 开始的整数数组 tasks ，其中 tasks[i] 表示任务的难度级别。在每一轮中，你可以完成 2 个或者 3 个 相同难度级别 的任务。

返回完成所有任务需要的 最少 轮数，如果无法完成所有任务，返回 -1 。
https://leetcode.cn/problems/minimum-rounds-to-complete-all-tasks/description/
*/
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut count_hash: HashMap<i32, i32> = HashMap::new();
        for task in tasks {
            *count_hash.entry(task).or_insert(0) += 1;
        }

        let mut ans = 0;
        for (_, value) in count_hash {
            if value == 1 {
                return -1;
            }
            if value % 3 == 0 {
                ans += value / 3;
            } else {
                ans += value / 3 + 1;
            }
        }
        ans
    }
}

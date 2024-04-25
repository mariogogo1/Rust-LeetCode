/**
56. 合并区间

以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。

https://leetcode.cn/problems/merge-intervals/description/
*/
pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut ans = Vec::new();
        ans.push(intervals[0].clone());

        for i in 1..intervals.len() {
            let mut end = ans.last_mut().unwrap()[1];
            if end >= intervals[i][0] {
                end = end.max(intervals[i][1]);
                ans.last_mut().unwrap()[1] = end;
            } else {
                ans.push(intervals[i].clone());
            }
        }

        ans
    }
}

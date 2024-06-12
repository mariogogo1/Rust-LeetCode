/**
3169. 无需开会的工作日

给你一个正整数 days，表示员工可工作的总天数（从第 1 天开始）。另给你一个二维数组 meetings，长度为 n，其中 meetings[i] = [start_i, end_i] 表示第 i 次会议的开始和结束天数（包含首尾）。

返回员工可工作且没有安排会议的天数。

注意：会议时间可能会有重叠。

https://leetcode.cn/problems/count-days-without-meetings/description/
*/
pub struct Solution;
impl Solution {
    pub fn count_days(mut days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut start = meetings[0][0];
        let mut end = meetings[0][1];

        for i in 1..meetings.len() {
            days -= end - start + 1;

            start = meetings[i][0].max(end + 1);
            end = meetings[i][1].max(end);
        }
        days -= end - start + 1;
        days
    }
}

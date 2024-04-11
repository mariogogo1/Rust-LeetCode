/**
435. 无重叠区间

给定一个区间的集合 intervals ，其中 intervals[i] = [starti, endi] 。返回 需要移除区间的最小数量，使剩余区间互不重叠 。

https://leetcode.cn/problems/longest-increasing-subsequence/description/
*/
/// 435. 无重叠区间
/// 646. 最长数对链
/// 以上兩題概念一模一樣
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut first = i32::MIN;
        let mut ans = 0;
        for interval in intervals {
            if first <= interval[0] {
                first = interval[1];
                ans += 1;
            }
        }
        n as i32 - ans
    }
}

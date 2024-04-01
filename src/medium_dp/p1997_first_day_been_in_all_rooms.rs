/**
1997. 访问完所有房间的第一天

你需要访问 n 个房间，房间从 0 到 n - 1 编号。同时，每一天都有一个日期编号，从 0 开始，依天数递增。你每天都会访问一个房间。

最开始的第 0 天，你访问 0 号房间。给你一个长度为 n 且 下标从 0 开始 的数组 nextVisit 。在接下来的几天中，你访问房间的 次序 将根据下面的 规则 决定：

假设某一天，你访问 i 号房间。
如果算上本次访问，访问 i 号房间的次数为 奇数 ，那么 第二天 需要访问 nextVisit[i] 所指定的房间，其中 0 <= nextVisit[i] <= i 。
如果算上本次访问，访问 i 号房间的次数为 偶数 ，那么 第二天 需要访问 (i + 1) mod n 号房间。
请返回你访问完所有房间的第一天的日期编号。题目数据保证总是存在这样的一天。由于答案可能很大，返回对 109 + 7 取余后的结果。

提示：
n == nextVisit.length
2 <= n <= 105
0 <= nextVisit[i] <= i

https://leetcode.cn/problems/first-day-where-you-have-been-in-all-the-rooms/description/
*/
pub struct Solution;

/// 其中 0 <= nextVisit[i] <= i 非常關鍵
/// => 第一次到達i時，[0,i-1) 都是偶數， [0,i-1)前面是完全重新來過的狀態
///
/// dp[i] 設定為第一次到達 i 的總天數 ，
/// 從 偶數次 i-1 到 奇數次 i 需要一天
/// 從 奇數次 i 到 第一次(因為視同重新開始)next_visit[i] 需要一天
/// 從 第一次(因為視同重新開始)next_visit[i] 到 偶數次i 需要 dp[i] - dp[next_visit[i]]

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let n_usize = next_visit.len();
        let mut dp: Vec<i64> = vec![0; n_usize];
        for i in 1..n_usize {
            dp[i] = 2 * dp[i - 1] - dp[next_visit[i - 1] as usize] + 2;
            dp[i] %= Self::VAL_MOD;
        }

        if dp[n_usize - 1] < 0 {
            dp[n_usize - 1] += Self::VAL_MOD;
        }

        return dp[n_usize - 1] as i32;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 0]), 2);
        assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 0, 2]), 6);
        assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 0, 1, 0]), 12);
    }
    #[test]
    fn test_case() {}
}

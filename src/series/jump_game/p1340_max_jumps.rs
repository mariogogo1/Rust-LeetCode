/**
1340. 跳跃游戏 V

给你一个整数数组 arr 和一个整数 d 。每一步你可以从下标 i 跳到：

i + x ，其中 i + x < arr.length 且 0 < x <= d 。
i - x ，其中 i - x >= 0 且 0 < x <= d 。
除此以外，你从下标 i 跳到下标 j 需要满足：arr[i] > arr[j] 且 arr[i] > arr[k] ，其中下标 k 是所有 i 到 j 之间的数字（更正式的，min(i, j) < k < max(i, j)）。

你可以选择数组的任意下标开始跳跃。请你返回你 最多 可以访问多少个下标。

请注意，任何时刻你都不能跳到数组的外面。

https://leetcode.cn/problems/jump-game-v/description/
*/
// 個人覺得經典的DFS跟DP 混合問題
impl Solution {
    fn dfs(dp: &mut Vec<i32>, arr: &Vec<i32>, d: i32, idx: usize) -> i32 {
        if dp[idx] != 0 {
            return dp[idx];
        }
        let mut ans = 0;
        let l = 0.max(idx as i32 - d) as usize;
        let r = (arr.len() - 1).min(idx + d as usize);
        for i in (idx + 1)..=r {
            if arr[i] < arr[idx] {
                ans = ans.max(Self::dfs(dp, &arr, d, i))
            } else {
                break;
            }
        }
        for i in (l..idx).rev() {
            if arr[i] < arr[idx] {
                ans = ans.max(Self::dfs(dp, &arr, d, i))
            } else {
                break;
            }
        }

        dp[idx] = ans + 1;
        dp[idx]
    }

    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; arr.len()];

        let mut ans: i32 = 0;

        for idx in 0..arr.len() {
            ans = ans.max(Self::dfs(&mut dp, &arr, d, idx));
        }
        ans
    }
}

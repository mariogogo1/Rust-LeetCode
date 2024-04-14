/**
1871. 跳跃游戏 VII

给你一个下标从 0 开始的二进制字符串 s 和两个整数 minJump 和 maxJump 。一开始，你在下标 0 处，且该位置的值一定为 '0' 。当同时满足如下条件时，你可以从下标 i 移动到下标 j 处：

i + minJump <= j <= min(i + maxJump, s.length - 1) 且
s[j] == '0'.
如果你可以到达 s 的下标 s.length - 1 处，请你返回 true ，否则返回 false 。

https://leetcode.cn/problems/jump-game-vii/description/
*/

// 用前綴和的方式紀錄一個點，有多少起點count可以跳到，當count > 0，且該點為0，紀錄為true。
impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let n = s.len();
        let mut dp = vec![false; n];
        dp[0] = true;
        let mut count = 0;
        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;

        for (i, ch) in s.chars().enumerate() {
            if i >= min_jump {
                if dp[i - min_jump] {
                    count += 1;
                }
            }

            if i >= max_jump + 1 {
                if dp[i - max_jump - 1] {
                    count -= 1;
                }
            }

            if count > 0 && ch == '0' {
                dp[i] = true;
            }
        }

        dp[n - 1]
    }
}

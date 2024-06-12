/**
2938. 区分黑球与白球

桌子上有 n 个球，每个球的颜色不是黑色，就是白色。

给你一个长度为 n 、下标从 0 开始的二进制字符串 s，其中 1 和 0 分别代表黑色和白色的球。

在每一步中，你可以选择两个相邻的球并交换它们。

返回「将所有黑色球都移到右侧，所有白色球都移到左侧所需的 最小步数」。

https://leetcode.cn/problems/separate-black-and-white-balls/description/
*/

pub struct Solution;
impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut l = 0 as usize;
        let n = s.len();
        let mut ans = 0 as i64;

        for r in 0..n {
            if s[r] == '0' {
                let ch = s[r];
                s[r] = s[l];
                s[l] = ch;
                ans += (r - l) as i64;
                l += 1;
            }
        }

        ans
    }
}

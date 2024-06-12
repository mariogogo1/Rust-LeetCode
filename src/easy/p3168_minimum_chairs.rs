/**
3168. 候诊室中的最少椅子数

给你一个字符串 s，模拟每秒钟的事件 i：

如果 s[i] == 'E'，表示有一位顾客进入候诊室并占用一把椅子。
如果 s[i] == 'L'，表示有一位顾客离开候诊室，从而释放一把椅子。
返回保证每位进入候诊室的顾客都能有椅子坐的 最少 椅子数，假设候诊室最初是 空的 。

https://leetcode.cn/problems/minimum-number-of-chairs-in-a-waiting-room/description/
*/

pub struct Solution;
impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let s = s.chars();
        let mut ans = 0;
        let mut max_v = 0;
        for ch in s {
            if ch == 'E' {
                max_v += 1;
            } else {
                max_v -= 1;
            }
            ans = ans.max(max_v);
        }
        ans
    }
}

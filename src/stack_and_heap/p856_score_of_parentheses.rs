/**
856. 括号的分数

给定一个平衡括号字符串 S，按下述规则计算该字符串的分数：

() 得 1 分。
AB 得 A + B 分，其中 A 和 B 是平衡括号字符串。
(A) 得 2 * A 分，其中 A 是平衡括号字符串。

https://leetcode.cn/problems/score-of-parentheses/description/
*/

pub struct Solution;
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut stack = Vec::new();
        let mut ans = 0;
        for i in 0..s.len() {
            let count = 0;
            if s[i] == '(' {
                stack.push(0);
            } else {
                let mut x = 0;
                if let Some(val) = stack.pop() {
                    if val == 0 {
                        x = 1;
                    } else {
                        x = val * 2;
                    }
                }
                if let Some(val) = stack.last_mut() {
                    *val += x;
                } else {
                    ans += x;
                }
            }
        }
        ans
    }
}

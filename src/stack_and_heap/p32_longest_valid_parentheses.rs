/**
32. 最长有效括号

给你一个只包含 '(' 和 ')' 的字符串，找出最长有效（格式正确且连续）括号子串的长度。

0 <= s.length <= 3 * 104
s[i] 为 '(' 或 ')'

https://leetcode.cn/problems/longest-valid-parentheses/description/
*/

pub struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<(char, i32)> = Vec::new(); // 使用元组 (字符, 计数) 来代替两个 Vec
        let mut ans = 0;
        let mut count = 0; //用來計算左括號'('前面，"有幾個有效的連續括號"

        for var in s.chars() {
            if var == '(' {
                stack.push(('(', count));
                count = 0; // 重置计数
            } else if var == ')' {
                if let Some(('(', prev_count)) = stack.pop() {
                    count += 2 + prev_count; // 更新计数，加上之前的计数并加上当前的有效括号
                    ans = ans.max(count);
                } else {
                    count = 0; // 沒有多出來的左括號了，連續符號被右括號截斷了，計數歸零
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);

        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }
    #[test]
    fn test_case() {}
}

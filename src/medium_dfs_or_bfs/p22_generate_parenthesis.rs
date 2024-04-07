/**
22. 括号生成

数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。

https://leetcode.cn/problems/generate-parentheses/description/
*/
pub struct Solution;

impl Solution {
    fn traceback(left: i32, right: i32, one_ans: String, ans: &mut Vec<String>) {
        if left == 0 {
            let mut one_ans = one_ans + &")".repeat(right as usize);
            ans.push(one_ans);
        } else if left == right {
            let one_ans = one_ans + "(";
            Self::traceback(left - 1, right, one_ans, ans);
        } else if left < right {
            let ans1 = one_ans.clone() + "(";
            Self::traceback(left - 1, right, ans1, ans);
            let ans2 = one_ans + ")";
            Self::traceback(left, right - 1, ans2, ans);
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        Self::traceback(n, n, String::new(), &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case() {}
}

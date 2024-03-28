/**
227. 基本计算器 II
面试题 16.26. 计算器

给定一个包含正整数、加(+)、减(-)、乘(*)、除(/)的算数表达式(括号除外)，计算其结果。

表达式仅包含非负整数，+， - ，*，/ 四种运算符和空格  。 整数除法仅保留整数部分。

https://leetcode.cn/problems/calculator-lcci/description/
https://leetcode.cn/problems/basic-calculator-ii/description/
*/
/// 這題比較考古題的做法是使用 stack 完成，這裡提供一個直接計算的做法

pub struct Solution;
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut ans: i64 = 0;
        let mut num: i64 = 1;
        let mut temp_num: i64 = 0;
        let mut sign: i64 = 1;
        let mut mult = true;

        for ch in s.chars() {
            match ch {
                '+' | '-' | '*' | '/' => {
                    if mult {
                        num *= temp_num;
                    } else if temp_num != 0 {
                        num /= temp_num;
                    } else {
                        num = 0;
                    }
                    temp_num = 0;
                    mult = true;

                    if ch == '+' || ch == '-' {
                        ans += sign * num;
                        sign = if ch == '+' { 1 } else { -1 };
                        num = 1;
                    } else if ch == '*' {
                        mult = true;
                    } else if ch == '/' {
                        mult = false;
                    }
                }
                '0'..='9' => {
                    let x = (ch as u32 - '0' as u32) as i64;
                    temp_num *= 10;
                    temp_num += x;
                }
                ' ' => continue,
                _ => todo!(),
            }
        }

        if mult {
            num *= temp_num;
        } else if temp_num != 0 {
            num /= temp_num;
        } else {
            num = 0;
        }

        ans += sign * num;
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::calculate("3+2*2".to_string()), 7);
        assert_eq!(Solution::calculate(" 3+5 / 2 ".to_string()), 5);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::calculate(" 0-0 ".to_string()), 0);
        assert_eq!(
            Solution::calculate(" 0-2147483647 ".to_string()),
            -2147483647
        );
    }
}

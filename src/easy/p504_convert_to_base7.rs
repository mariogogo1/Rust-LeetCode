/**
504. 七进制数

给定一个整数 num，将其转化为 7 进制，并以字符串形式输出。

https://leetcode.cn/problems/base-7/description/
*/

///
///https://rustwiki.org/zh-CN/std/fmt/
///

pub struct Solution;

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let mut ans = String::from("");
        let mut n = num;
        if num < 0 {
            n = -num;
        } else if num == 0 {
            return "0".to_string();
        }
        while n > 0 {
            let i = n % 7;
            ans.push((i as u8 + '0' as u8) as char);
            n /= 7;
        }
        if num < 0 {
            ans.push('-');
        }

        return ans.chars().rev().collect::<String>();
    }

    pub fn convert_to_base7_recursion(num: i32) -> String {
        if num < 0 {
            return format!("-{}", Self::convert_to_base7_recursion(-num));
        }
        if num < 7 {
            return format!("{}", num);
        }
        return format!(
            "{}{}",
            Self::convert_to_base7_recursion(num / 7),
            Self::convert_to_base7_recursion(num % 7)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::convert_to_base7(5), String::from("5"));
        assert_eq!(Solution::convert_to_base7(-100), String::from("-202"));
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::convert_to_base7(0), String::from("0"));
    }
}

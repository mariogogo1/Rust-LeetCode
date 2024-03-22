/**
402. 移掉 K 位数字

给你一个以字符串表示的非负整数 num 和一个整数 k ，移除这个数中的 k 位数字，使得剩下的数字最小。请你以字符串形式返回这个最小的数字。

Hint:
1 <= k <= num.length <= 105
num 仅由若干位数字（0 - 9）组成
除了 0 本身之外，num 不含任何前导零

https://leetcode.cn/problems/remove-k-digits/description/
*/

pub struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        if k >= num.len() as i32 {
            return "0".to_string();
        }

        let mut ans: Vec<char> = Vec::new();
        let mut k_mut = k;
        let num_chars: Vec<char> = num.chars().collect();

        for i in 0..num_chars.len() {
            while !ans.is_empty() && *ans.last().unwrap() > num_chars[i] && k_mut > 0 {
                ans.pop();
                k_mut -= 1;
            }
            ans.push(num_chars[i]);
        }

        while k_mut > 0 {
            ans.pop();
            k_mut -= 1;
        }

        // 转换为字符串，并移除前导零
        let trimmed_ans = ans
            .iter()
            .collect::<String>()
            .trim_start_matches('0')
            .to_string();

        if trimmed_ans.is_empty() {
            return "0".to_string();
        }

        trimmed_ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::remove_kdigits("1432219".to_string(), 3),
            "1219".to_string()
        );

        assert_eq!(
            Solution::remove_kdigits("10200".to_string(), 1),
            "200".to_string()
        );
        assert_eq!(
            Solution::remove_kdigits("10".to_string(), 2),
            "0".to_string()
        );
        assert_eq!(
            Solution::remove_kdigits("10".to_string(), 1),
            "0".to_string()
        );
    }
    #[test]
    fn test_case() {
        assert_eq!(
            Solution::remove_kdigits("112".to_string(), 1),
            "11".to_string()
        );
    }
}

/**
17. 电话号码的字母组合

给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。

给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。

https://leetcode.cn/problems/letter-combinations-of-a-phone-number/description/
*/
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    fn dfs(map: &HashMap<char, &str>, ans: &mut Vec<String>, one_ans: &str, digit: &str) {
        if digit.is_empty() {
            ans.push(one_ans.to_string());
            return;
        }
        let letter = digit.chars().next().unwrap();
        let choice = map.get(&letter).unwrap_or(&"");
        for c in choice.chars() {
            let mut new_one_ans = one_ans.to_string();
            new_one_ans.push(c);
            Self::dfs(map, ans, &new_one_ans, &digit[1..]);
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let letter_combinations_map: HashMap<char, &str> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .iter()
        .cloned()
        .collect();

        let mut ans = Vec::new();

        if !digits.is_empty() {
            Self::dfs(&letter_combinations_map, &mut ans, "", digits.as_str());
        }

        ans
    }
}

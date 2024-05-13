/**
3136. 有效单词

有效单词 需要满足以下几个条件：

至少 包含 3 个字符。
由数字 0-9 和英文大小写字母组成。（不必包含所有这类字符。）
至少 包含一个 元音字母 。
至少 包含一个 辅音字母 。
给你一个字符串 word 。如果 word 是一个有效单词，则返回 true ，否则返回 false 。

注意：

'a'、'e'、'i'、'o'、'u' 及其大写形式都属于 元音字母 。
英文中的 辅音字母 是指那些除元音字母之外的字母。

https://leetcode.cn/problems/valid-word/description/
*/

pub struct Solution;
impl Solution {
    pub fn is_valid(word: String) -> bool {
        let mut has_aeiou = false;
        let mut has_other = false;
        for ch in word.chars() {
            match ch {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => has_aeiou = true,
                '0'..='9' => continue,
                '@' | '#' | '$' => return false,
                _ => has_other = true,
            }
        }

        word.len() >= 3 && has_aeiou && has_other
    }
}

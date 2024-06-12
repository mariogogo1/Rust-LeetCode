/**
3144. 分割字符频率相等的最少子字符串

给你一个字符串 s ，你需要将它分割成一个或者更多的 平衡 子字符串。比方说，s == "ababcc" 那么 ("abab", "c", "c") ，("ab", "abc", "c") 和 ("ababcc") 都是合法分割，但是 ("a", "bab", "cc") ，("aba", "bc", "c") 和 ("ab", "abcc") 不是，不平衡的子字符串用粗体表示。

请你返回 s 最少 能分割成多少个平衡子字符串。

注意：一个 平衡 字符串指的是字符串中所有字符出现的次数都相同。

https://leetcode.cn/problems/minimum-substring-partition-of-equal-character-frequency/description/
*/

pub struct Solution;
// 時間有點久，要再研究一下有沒有更快的判斷BALANCE的方式!
impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let n = s.len();
        let s_vec: Vec<char> = s.chars().collect();

        let mut dp = vec![0; n + 1];

        for i in 1..=n {
            let mut min_v = i32::MAX;
            let mut count = vec![0; 26];
            for j in (0..i).rev() {
                count[(s_vec[j] as u8 - b'a') as usize] += 1;
                if Self::is_balance_string(&count) {
                    min_v = min_v.min(dp[j] + 1);
                }
            }
            dp[i] = min_v;
        }

        dp[n]
    }

    fn is_balance_string(char_count_vec: &Vec<i32>) -> bool {
        let mut s = 0;
        for i in 0..char_count_vec.len() {
            if char_count_vec[i] == 0 {
                continue;
            }
            if s == 0 {
                s = char_count_vec[i];
            }
            if char_count_vec[i] != s {
                return false;
            }
        }
        true
    }
}

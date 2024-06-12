/**
3170. 删除星号以后字典序最小的字符串

给你一个字符串 s 。它可能包含任意数量的 '*' 字符。你的任务是删除所有的 '*' 字符。

当字符串还存在至少一个 '*' 字符时，你可以执行以下操作：

删除最左边的 '*' 字符，同时删除该星号字符左边一个字典序 最小 的字符。如果有多个字典序最小的字符，你可以删除它们中的任意一个。
请你返回删除所有 '*' 字符以后，剩余字符连接而成的
字典序最小的字符串。

https://leetcode.cn/problems/lexicographically-minimum-string-after-removing-stars/
*/
pub struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut ans = Vec::new();
        let mut s = s.chars().collect::<Vec<char>>();
        let mut min_heap: BinaryHeap<Reverse<(u8, Reverse<usize>)>> = BinaryHeap::new();
        let n = s.len();

        for i in 0..n {
            if s[i] == '*' {
                if let Some(Reverse((_, Reverse(idx)))) = min_heap.pop() {
                    s[idx] = '*';
                }
            } else {
                min_heap.push(Reverse((s[i] as u8, Reverse(i)))); // **很神奇!!
            }
        }
        for ch in s {
            if ch != '*' {
                ans.push(ch);
            }
        }

        ans.iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::clear_stars("dk**".to_string()), "".to_string());
    }
    #[test]
    fn test_case() {}
}

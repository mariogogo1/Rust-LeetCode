/**
68. 文本左右对齐

给定一个单词数组 words 和一个长度 maxWidth ，重新排版单词，使其成为每行恰好有 maxWidth 个字符，且左右两端对齐的文本。

你应该使用 “贪心算法” 来放置给定的单词；也就是说，尽可能多地往每行中放置单词。必要时可用空格 ' ' 填充，使得每行恰好有 maxWidth 个字符。

要求尽可能均匀分配单词间的空格数量。如果某一行单词间的空格不能均匀分配，则左侧放置的空格数要多于右侧的空格数。

文本的最后一行应为左对齐，且单词之间不插入额外的空格。

注意:

单词是指由非空格字符组成的字符序列。
每个单词的长度大于 0，小于等于 maxWidth。
输入单词数组 words 至少包含一个单词。

https://leetcode.cn/problems/text-justification/description/?envType=study-plan-v2&envId=top-interview-150
*/

pub struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let mut accumulated = 0;
        let (mut start, mut end) = (0, 0);
        let mut i = 0 as usize;

        while i < words.len() {
            if accumulated == 0 {
                accumulated -= 1; // 第一個字沒有前置空格
                accumulated += words[i].len() as i32 + 1;
                start = i;
            } else {
                accumulated += words[i].len() as i32 + 1;
            }

            if accumulated > max_width {
                end = i;

                let s = if start == end - 1 {
                    Self::alignment(
                        &words[start..end],
                        accumulated - words[i].len() as i32 - 1,
                        max_width,
                        true,
                    )
                } else {
                    Self::alignment(
                        &words[start..end],
                        accumulated - words[i].len() as i32 - 1,
                        max_width,
                        false,
                    )
                };
                ans.push(s);
                i -= 1;
                accumulated = 0;
            }
            i += 1;
        }

        let s = Self::alignment(&words[start..], accumulated, max_width, true);
        ans.push(s);
        ans
    }

    fn alignment(words: &[String], length: i32, max_width: i32, left: bool) -> String {
        let mut ans = String::new();
        if left {
            for i in 0..words.len() - 1 {
                ans += &words[i];
                ans += " ";
            }
            ans += &words[words.len() - 1];

            for _ in 0..max_width - length {
                ans += " ";
            }
        } else {
            let space_count = (words.len() - 1) as i32;
            let space_length = (max_width - length) / space_count;
            let mut more_space = (max_width - length) % space_count;

            for i in 0..words.len() - 1 {
                ans += &words[i];
                for _ in 0..=space_length {
                    ans += " ";
                }
                if more_space > 0 {
                    more_space -= 1;
                    ans += " ";
                }
            }
            ans += &words[words.len() - 1];
        }

        ans
    }
}

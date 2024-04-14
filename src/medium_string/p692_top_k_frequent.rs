/**
692. 前K个高频单词

给定一个单词列表 words 和一个整数 k ，返回前 k 个出现次数最多的单词。

返回的答案应该按单词出现频率由高到低排序。如果不同的单词有相同出现频率， 按字典顺序 排序。

https://leetcode.cn/problems/top-k-frequent-words/description/
*/

pub struct Solution;

// 參考347題
// 本題關鍵在於寫出字典序排序方式

use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut word_count: HashMap<String, i32> = HashMap::new();

        // 計算每個單詞的出現次數
        for word in &words {
            let count = word_count.entry(word.to_string()).or_insert(0);
            *count += 1;
        }

        // 使用來排序單詞的出現次數
        let mut record: Vec<(i32, String)> = Vec::new();
        for (word, count) in word_count {
            record.push((count, word));
        }

        record.sort_unstable_by(|a, b| {
            if a.0 != b.0 {
                return b.0.cmp(&a.0);
            } else {
                // 比較 a.1.chars() 跟 b.1.chars()的大小,只要分出大小跳出迴圈
                let n = a.1.len().min(b.1.len());
                for i in 0..n {
                    if a.1.chars().nth(i) != b.1.chars().nth(i) {
                        return a.1.chars().nth(i).cmp(&b.1.chars().nth(i));
                    }
                }
                return a.1.len().cmp(&b.1.len());
            }
        });

        // 取出前 k 個出現次數最多的單詞
        let mut result: Vec<String> = Vec::new();
        for i in 0..k as usize {
            result.push(record[i].1.clone());
        }

        result
    }
}

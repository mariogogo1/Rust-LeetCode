/**
LCR 114. 火星词典

现有一种使用英语字母的外星文语言，这门语言的字母顺序与英语顺序不同。

给定一个字符串列表 words ，作为这门语言的词典，words 中的字符串已经 按这门新语言的字母顺序进行了排序 。

请你根据该词典还原出此语言中已知的字母顺序，并 按字母递增顺序 排列。若不存在合法字母顺序，返回 "" 。若存在多种可能的合法字母顺序，返回其中 任意一种 顺序即可。

字符串 s 字典顺序小于 字符串 t 有两种情况：

在第一个不同字母处，如果 s 中的字母在这门外星语言的字母顺序中位于 t 中字母之前，那么 s 的字典顺序小于 t 。
如果前面 min(s.length, t.length) 字母都相同，那么 s.length < t.length 时，s 的字典顺序也小于 t 。

https://leetcode.cn/problems/Jf1JuT/description/
*/

pub struct Solution;
use std::collections::{HashMap, VecDeque};

impl Solution {
    fn compare(w1: &str, w2: &str) -> (u8, u8, bool) {
        let mut s: u8 = w1.as_bytes()[0];
        let mut b: u8 = w2.as_bytes()[0];
        let mut wrong: bool = true;

        let n = w1.len().min(w2.len());
        for i in 0..n {
            if w1.as_bytes()[i] != w2.as_bytes()[i] {
                s = w1.as_bytes()[i];
                b = w2.as_bytes()[i];
                return (s, b, true);
            }
        }
        if w1.len() > w2.len() {
            wrong = false;
        }
        (s, b, wrong)
    }

    pub fn alien_order(words: Vec<String>) -> String {
        let mut graph: HashMap<u8, Vec<u8>> = HashMap::new(); // 比小的大的有哪些
        let mut valid: HashMap<u8, usize> = HashMap::new(); // 比大的小的数量 0 就是当前最小的
        let mut ans: Vec<u8> = Vec::new();

        for i in 1..words.len() {
            let (small_ch, big_ch, wrong) = Self::compare(&words[i - 1], &words[i]);
            if !wrong {
                return "".to_string();
            }
            let small_entry = graph.entry(small_ch).or_insert(Vec::new());
            if small_ch != big_ch {
                small_entry.push(big_ch);
                *valid.entry(big_ch).or_insert(0) += 1;
            }
        }

        for word in &words {
            for &ch in word.as_bytes() {
                valid.entry(ch).or_insert(0);
            }
        }

        let mut queue: VecDeque<u8> = VecDeque::new();
        for (&k, &v) in &valid {
            if v == 0 {
                queue.push_back(k);
                ans.push(k);
            }
        }

        while let Some(s) = queue.pop_front() {
            if let Some(neighbors) = graph.get(&s) {
                for &v in neighbors {
                    *valid.get_mut(&v).unwrap() -= 1;
                    if *valid.get(&v).unwrap() == 0 {
                        queue.push_back(v);
                        ans.push(v);
                    }
                }
            }
        }

        if ans.len() == valid.len() {
            String::from_utf8(ans).unwrap()
        } else {
            "".to_string()
        }
    }
}

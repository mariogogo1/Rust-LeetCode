/**
208. 实现 Trie (前缀树)

Trie（发音类似 "try"）或者说 前缀树 是一种树形数据结构，用于高效地存储和检索字符串数据集中的键。这一数据结构有相当多的应用情景，例如自动补完和拼写检查。

请你实现 Trie 类：

Trie() 初始化前缀树对象。
void insert(String word) 向前缀树中插入字符串 word 。
boolean search(String word) 如果字符串 word 在前缀树中，返回 true（即，在检索之前已经插入）；否则，返回 false 。
boolean startsWith(String prefix) 如果之前已经插入的字符串 word 的前缀之一为 prefix ，返回 true ；否则，返回 false 。


https://leetcode.cn/problems/implement-trie-prefix-tree/description/
*/

#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 26],
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        self.insert_recursive(word.as_bytes());
    }

    fn insert_recursive(&mut self, word: &[u8]) {
        if word.is_empty() {
            self.end = true;
            return;
        }
        let ch = word[0];
        let ch_idx = (ch - b'a') as usize;
        self.next[ch_idx].get_or_insert_with(|| Box::new(Trie::new()));

        self.next[ch_idx]
            .as_mut()
            .unwrap()
            .insert_recursive(&word[1..]);
    }

    fn search(&self, word: String) -> bool {
        self.search_recursive(word.as_bytes(), false)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.search_recursive(prefix.as_bytes(), true)
    }

    fn search_recursive(&self, word: &[u8], find_prefix: bool) -> bool {
        if word.is_empty() {
            if find_prefix {
                return true;
            }
            return self.end;
        }
        let ch = word[0];
        let ch_idx = (ch - b'a') as usize;
        if let Some(nxt_trie) = self.next[ch_idx].as_ref() {
            return nxt_trie.search_recursive(&word[1..], find_prefix);
        }
        return false;
    }
}

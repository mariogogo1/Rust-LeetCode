/**
211. 添加与搜索单词 - 数据结构设计

请你设计一个数据结构，支持 添加新单词 和 查找字符串是否与任何先前添加的字符串匹配 。

实现词典类 WordDictionary ：

WordDictionary() 初始化词典对象
void addWord(word) 将 word 添加到数据结构中，之后可以对它进行匹配
bool search(word) 如果数据结构中存在字符串与 word 匹配，则返回 true ；否则，返回  false 。word 中可能包含一些 '.' ，每个 . 都可以表示任何一个字母。

https://leetcode.cn/problems/design-add-and-search-words-data-structure/description/
*/

#[derive(Default)]
struct WordDictionary {
    next: [Option<Box<WordDictionary>>; 26],
    end: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Default::default()
    }

    fn add_word(&mut self, word: String) {
        self.add_word_recursive(word.as_bytes());
    }

    fn add_word_recursive(&mut self, word: &[u8]) {
        if word.is_empty() {
            self.end = true;
            return;
        }
        let ch = word[0];
        let ch_idx = (ch - b'a') as usize;
        self.next[ch_idx].get_or_insert_with(|| Box::new(WordDictionary::new()));

        self.next[ch_idx]
            .as_mut()
            .unwrap()
            .add_word_recursive(&word[1..]);
    }

    fn search(&self, word: String) -> bool {
        self.search_recursive(word.as_bytes())
    }

    fn search_recursive(&self, word: &[u8]) -> bool {
        if word.is_empty() {
            return self.end;
        }
        let ch = word[0];
        if ch == b'.' {
            let mut ans = false;
            for i in 0..26 {
                if let Some(nxt_trie) = self.next[i].as_ref() {
                    ans |= nxt_trie.search_recursive(&word[1..]);
                }
            }
            return ans;
        }
        let ch_idx = (ch - b'a') as usize;
        if let Some(nxt_trie) = self.next[ch_idx].as_ref() {
            return nxt_trie.search_recursive(&word[1..]);
        }
        return false;
    }
}

/**

212. 单词搜索 II

给定一个 m x n 二维字符网格 board 和一个单词（字符串）列表 words， 返回所有二维网格上的单词 。

单词必须按照字母顺序，通过 相邻的单元格 内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母在一个单词中不允许被重复使用。

https://leetcode.cn/problems/word-search-ii/description/
*/
pub struct Solution;

/// 這題要思考起來就是字典樹，並不算困難，可是代碼量有點多 最好先寫前置題目 208 跟 79
/// 並且熟悉剪枝跟DFS模板
/// 這題的剪枝在查過某個單字的最後一個字母時，會把該字母從字典樹中移除，從而減少字典的大小

struct Directions {
    steps: Vec<(i32, i32)>,
}

impl Directions {
    pub fn new() -> Self {
        return Directions {
            steps: vec![(1, 0), (0, 1), (-1, 0), (0, -1)],
        };
    }
}

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
}
impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut word_dict = Trie::new();
        let dirs = Directions::new();

        for word in words {
            word_dict.insert(word);
        }

        let mut ans = Vec::new();
        let mut one_ans = Vec::new();

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                Self::dfs(
                    &mut board,
                    i,
                    j,
                    &dirs,
                    &mut word_dict,
                    &mut one_ans,
                    &mut ans,
                );
            }
        }

        ans
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        x: usize,
        y: usize,
        dirs: &Directions,
        word_dict: &mut Trie,
        one_ans: &mut Vec<char>,
        ans: &mut Vec<String>,
    ) {
        if let Some(mut nxt_trie) = word_dict.next[(board[x][y] as u8 - b'a') as usize].as_mut() {
            let ch = board[x][y];
            one_ans.push(ch);
            board[x][y] = '#';
            if nxt_trie.end {
                ans.push(one_ans.iter().collect());
                nxt_trie.end = false;
            }
            for dir in &dirs.steps {
                let n_x = x as i32 + dir.0;
                let n_y = y as i32 + dir.1;

                if n_x >= 0
                    && n_x < board.len() as i32
                    && n_y >= 0
                    && n_y < board[0].len() as i32
                    && board[n_x as usize][n_y as usize] != '#'
                {
                    Self::dfs(
                        board,
                        n_x as usize,
                        n_y as usize,
                        dirs,
                        &mut nxt_trie,
                        one_ans,
                        ans,
                    );
                }
            }

            one_ans.pop();
            board[x][y] = ch;

            //查過的單字就不查了 神奇剪枝!! 若改用HASHSET 可讀性會更好
            let mut cut = true;
            for i in 0..26 {
                if nxt_trie.next[i].is_some() {
                    cut = false;
                    break;
                }
            }
            if cut {
                word_dict.next[(board[x][y] as u8 - b'a') as usize] = None;
            }
        } else {
            return;
        }
    }
}

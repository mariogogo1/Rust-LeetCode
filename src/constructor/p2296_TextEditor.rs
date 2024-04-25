/**
2296. 设计一个文本编辑器

请你设计一个带光标的文本编辑器，它可以实现以下功能：

添加：在光标所在处添加文本。
删除：在光标所在处删除文本（模拟键盘的删除键）。
移动：将光标往左或者往右移动。
当删除文本时，只有光标左边的字符会被删除。光标会留在文本内，也就是说任意时候 0 <= cursor.position <= currentText.length 都成立。

请你实现 TextEditor 类：

TextEditor() 用空文本初始化对象。
void addText(string text) 将 text 添加到光标所在位置。添加完后光标在 text 的右边。
int deleteText(int k) 删除光标左边 k 个字符。返回实际删除的字符数目。
string cursorLeft(int k) 将光标向左移动 k 次。返回移动后光标左边 min(10, len) 个字符，其中 len 是光标左边的字符数目。
string cursorRight(int k) 将光标向右移动 k 次。返回移动后光标左边 min(10, len) 个字符，其中 len 是光标左边的字符数目。

https://leetcode.cn/problems/design-a-text-editor/description/
*/
#[derive(Default)]
struct TextEditor {
    before_cursor: Vec<char>,
    after_cursor: Vec<char>,
}

impl TextEditor {
    fn new() -> Self {
        Default::default()
    }

    fn add_text(&mut self, text: String) {
        for c in text.chars() {
            self.before_cursor.push(c);
        }
    }

    fn delete_text(&mut self, mut k: i32) -> i32 {
        let mut count = 0;
        while !self.before_cursor.is_empty() && k > 0 {
            self.before_cursor.pop();
            count += 1;
            k -= 1;
        }
        count
    }

    fn cursor_left(&mut self, mut k: i32) -> String {
        while k > 0 {
            if let Some(c) = self.before_cursor.pop() {
                self.after_cursor.push(c);
            }
            k -= 1;
        }
        self.output_ans()
    }

    fn cursor_right(&mut self, mut k: i32) -> String {
        while k > 0 {
            if let Some(c) = self.after_cursor.pop() {
                self.before_cursor.push(c);
            }
            k -= 1;
        }
        self.output_ans()
    }

    fn output_ans(&self) -> String {
        let length = 10.min(self.before_cursor.len());
        let mut ans = Vec::new();
        let final_len = self.before_cursor.len() - 1;
        for i in (0..length).rev() {
            ans.push(self.before_cursor[final_len - i]);
        }

        ans.iter().collect()
    }
}

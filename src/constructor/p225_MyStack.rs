/**
225. 用队列实现栈

请你仅使用两个队列实现一个后入先出（LIFO）的栈，并支持普通栈的全部四种操作（push、top、pop 和 empty）。

实现 MyStack 类：

void push(int x) 将元素 x 压入栈顶。
int pop() 移除并返回栈顶元素。
int top() 返回栈顶元素。
boolean empty() 如果栈是空的，返回 true ；否则，返回 false 。

Hint:
你只能使用队列的标准操作 —— 也就是 push to back、peek/pop from front、size 和 is empty 这些操作。
你所使用的语言也许不支持队列。 你可以使用 list （列表）或者 deque（双端队列）来模拟一个队列 , 只要是标准的队列操作即可。
1 <= x <= 9
最多调用100 次 push、pop、top 和 empty
每次调用 pop 和 top 都保证栈不为空

https://leetcode.cn/problems/implement-stack-using-queues/description/
*/

/// 練習自己寫一個單向對列 MyQueue
/// RUST 標準庫裏面有內建VecDeque雙端對列
/// 都可以作為練習使用
struct MyQueue<T> {
    objs: Vec<T>,
}

impl<T> MyQueue<T> {
    pub fn new() -> Self {
        MyQueue { objs: Vec::new() }
    }
    pub fn push(&mut self, obj: T) {
        self.objs.push(obj);
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.objs.is_empty() {
            return None;
        }
        return Some(self.objs.remove(0));
    }
    pub fn peek(&self) -> Option<&T> {
        self.objs.get(0)
    }
    pub fn len(&self) -> usize {
        self.objs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.objs.is_empty()
    }
}

/// 關鍵想法：把對列當作一個環，每次入對列的的時候都進行O(N)操作，調次序
// 用隊列實現棧功能
struct MyStack {
    objs: MyQueue<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            objs: MyQueue::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.objs.push(x);
        for i in 0..self.objs.len() - 1 {
            let y = self.objs.pop().unwrap();
            self.objs.push(y);
        }
    }

    fn pop(&mut self) -> i32 {
        self.objs.pop().unwrap()
    }

    fn top(&self) -> i32 {
        *self.objs.peek().unwrap()
    }

    fn empty(&self) -> bool {
        self.objs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case() {}
}

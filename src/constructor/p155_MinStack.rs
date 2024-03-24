/**
155. 最小栈

设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。

实现 MinStack 类:

MinStack() 初始化堆栈对象。
void push(int val) 将元素val推入堆栈。
void pop() 删除堆栈顶部的元素。
int top() 获取堆栈顶部的元素。
int getMin() 获取堆栈中的最小元素。

https://leetcode.cn/problems/min-stack/description/
*/
struct MinStack {
    stack: Vec<i32>,
    min_vec: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_vec: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_vec.is_empty() || val <= *self.min_vec.last().unwrap() {
            self.min_vec.push(val);
        } else {
            self.min_vec.push(*self.min_vec.last().unwrap());
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_vec.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_vec.last().unwrap()
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

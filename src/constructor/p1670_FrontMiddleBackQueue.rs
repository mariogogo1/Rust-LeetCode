/**
1670. 设计前中后队列

请你设计一个队列，支持在前，中，后三个位置的 push 和 pop 操作。

请你完成 FrontMiddleBack 类：

FrontMiddleBack() 初始化队列。
void pushFront(int val) 将 val 添加到队列的 最前面 。
void pushMiddle(int val) 将 val 添加到队列的 正中间 。
void pushBack(int val) 将 val 添加到队里的 最后面 。
int popFront() 将 最前面 的元素从队列中删除并返回值，如果删除之前队列为空，那么返回 -1 。
int popMiddle() 将 正中间 的元素从队列中删除并返回值，如果删除之前队列为空，那么返回 -1 。
int popBack() 将 最后面 的元素从队列中删除并返回值，如果删除之前队列为空，那么返回 -1 。
请注意当有 两个 中间位置的时候，选择靠前面的位置进行操作。比方说：

将 6 添加到 [1, 2, 3, 4, 5] 的中间位置，结果数组为 [1, 2, 6, 3, 4, 5] 。
从 [1, 2, 3, 4, 5, 6] 的中间位置弹出元素，返回 3 ，数组变为 [1, 2, 4, 5, 6] 。

https://leetcode.cn/problems/design-front-middle-back-queue/description/
*/
use std::collections::VecDeque;

#[derive(Default)]
struct FrontMiddleBackQueue {
    before: VecDeque<i32>,
    after: VecDeque<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self {
        Default::default()
    }

    fn push_front(&mut self, val: i32) {
        self.before.push_front(val);
        if self.before.len() == self.after.len() + 1 {
            if let Some(v) = self.before.pop_back() {
                self.after.push_front(v);
            }
        }
    }

    fn push_middle(&mut self, val: i32) {
        if self.before.len() == self.after.len() {
            self.after.push_front(val);
        } else {
            self.before.push_back(val);
        }
    }

    fn push_back(&mut self, val: i32) {
        self.after.push_back(val);
        if self.before.len() + 2 == self.after.len() {
            if let Some(v) = self.after.pop_front() {
                self.before.push_back(v);
            }
        }
    }

    fn pop_front(&mut self) -> i32 {
        if self.before.is_empty() {
            if self.after.is_empty() {
                return -1;
            } else {
                return self.after.pop_front().unwrap();
            }
        }
        if self.before.len() + 1 == self.after.len() {
            if let Some(v) = self.after.pop_front() {
                self.before.push_back(v);
            }
        }
        return self.before.pop_front().unwrap();
    }

    fn pop_middle(&mut self) -> i32 {
        if self.before.len() == self.after.len() {
            if self.before.is_empty() {
                return -1;
            }
            return self.before.pop_back().unwrap();
        }
        return self.after.pop_front().unwrap();
    }

    fn pop_back(&mut self) -> i32 {
        if self.after.is_empty() {
            return -1;
        }

        if self.before.len() == self.after.len() {
            if let Some(v) = self.before.pop_back() {
                self.after.push_front(v);
            }
        }

        return self.after.pop_back().unwrap();
    }
}

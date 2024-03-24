/**
895. 最大频率栈

设计一个类似堆栈的数据结构，将元素推入堆栈，并从堆栈中弹出出现频率最高的元素。

实现 FreqStack 类:

FreqStack() 构造一个空的堆栈。
void push(int val) 将一个整数 val 压入栈顶。
int pop() 删除并返回堆栈中出现频率最高的元素。
如果出现频率最高的元素不只一个，则移除并返回最接近栈顶的元素。

https://leetcode.cn/problems/implement-queue-using-stacks/description/
*/
/// 每種次數都做一個棧，即可POP最接近棧頂的最高頻率數字
use std::collections::HashMap;

struct FreqStack {
    frequency_map: HashMap<i32, usize>, // 查詢每個數字當前次數=>PUSH時，決定加入到第幾個棧。
    frequency_stacks: Vec<Vec<i32>>,    // 每個頻率次數的棧
}

impl FreqStack {
    fn new() -> Self {
        FreqStack {
            frequency_map: HashMap::new(),
            frequency_stacks: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let current_frequency = *self.frequency_map.get(&val).unwrap_or(&0);
        if self.frequency_stacks.len() == current_frequency {
            self.frequency_stacks.push(Vec::new());
        }

        *self.frequency_map.entry(val).or_insert(0) += 1;
        self.frequency_stacks[current_frequency].push(val);
    }

    fn pop(&mut self) -> i32 {
        if let Some(stack) = self.frequency_stacks.last_mut() {
            if let Some(value) = stack.pop() {
                *self.frequency_map.get_mut(&value).unwrap() -= 1;
                if stack.is_empty() {
                    self.frequency_stacks.pop();
                }
                return value;
            }
        }
        unreachable!();
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

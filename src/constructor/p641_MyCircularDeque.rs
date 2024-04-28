/**
641. 设计循环双端队列

设计实现双端队列。

实现 MyCircularDeque 类:

MyCircularDeque(int k) ：构造函数,双端队列最大为 k 。
boolean insertFront()：将一个元素添加到双端队列头部。 如果操作成功返回 true ，否则返回 false 。
boolean insertLast() ：将一个元素添加到双端队列尾部。如果操作成功返回 true ，否则返回 false 。
boolean deleteFront() ：从双端队列头部删除一个元素。 如果操作成功返回 true ，否则返回 false 。
boolean deleteLast() ：从双端队列尾部删除一个元素。如果操作成功返回 true ，否则返回 false 。
int getFront() )：从双端队列头部获得一个元素。如果双端队列为空，返回 -1 。
int getRear() ：获得双端队列的最后一个元素。 如果双端队列为空，返回 -1 。
boolean isEmpty() ：若双端队列为空，则返回 true ，否则返回 false  。
boolean isFull() ：若双端队列满了，则返回 true ，否则返回 false 。

https://leetcode.cn/problems/design-circular-deque/description/
*/
#[derive(Default)]
struct MyCircularDeque {
    capacity: usize,
    head: usize,
    tail: usize,
    nums: Vec<i32>,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        MyCircularDeque {
            capacity: k as usize,
            head: 0,
            tail: 0,
            nums: vec![0; k as usize + 1],
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        if self.head == 0 {
            self.head += self.nums.len();
        }
        self.head -= 1;
        self.nums[self.head] = value;
        return true;
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.nums[self.tail] = value;
        self.tail += 1;
        self.tail %= self.nums.len();
        return true;
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head += 1;
        self.head %= self.nums.len();
        return true;
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        if self.tail == 0 {
            self.tail += self.nums.len();
        }
        self.tail -= 1;
        return true;
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        return self.nums[self.head];
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        if self.tail == 0 {
            return *self.nums.last().unwrap();
        }
        return self.nums[self.tail - 1];
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn is_full(&self) -> bool {
        self.head == (self.tail + 1) % self.nums.len()
    }
}

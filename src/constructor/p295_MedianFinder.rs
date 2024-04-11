/**
295. 数据流的中位数
面试题 17.20. 连续中值

随机产生数字并传递给一个方法。你能否完成这个方法，在每次产生新值时，寻找当前所有值的中间值（中位数）并保存。

中位数是有序列表中间的数。如果列表长度是偶数，中位数则是中间两个数的平均值。

例如，

[2,3,4] 的中位数是 3

[2,3] 的中位数是 (2 + 3) / 2 = 2.5

设计一个支持以下两种操作的数据结构：

void addNum(int num) - 从数据流中添加一个整数到数据结构中。
double findMedian() - 返回目前所有元素的中位数。

https://leetcode.cn/problems/continuous-median-lcci/description/
*/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// 經典考題 維護大根堆根小根堆 維護數據流的中值
struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.max_heap.len() == self.min_heap.len() {
            // 先加入小根堆 從小根堆的堆頂移出最小的 ，再把那個值移入 大根堆
            self.min_heap.push(Reverse(num));
            if let Some(Reverse(value)) = self.min_heap.pop() {
                self.max_heap.push(value);
            }
        }
        // 先加入大根堆 從大根堆的堆頂移出最大的 ，再把那個值移入 小根堆
        self.max_heap.push(num);
        if let Some(value) = self.max_heap.pop() {
            self.min_heap.push(Reverse(value));
        }
    }

    fn find_median(&self) -> f64 {
        if self.max_heap.len() == self.min_heap.len() {
            if let Some(&value1) = self.max_heap.peek() {
                if let Some(&Reverse(value2)) = self.min_heap.peek() {
                    return (value1 + value2) as f64 / 2.0;
                }
            }
        }
        if let Some(&value) = self.max_heap.peek() {
            return value as f64;
        }
        0.0
    }
}

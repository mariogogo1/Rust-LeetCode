/**
352. 将数据流变为多个不相交区间

 给你一个由非负整数 a1, a2, ..., an 组成的数据流输入，请你将到目前为止看到的数字总结为不相交的区间列表。

实现 SummaryRanges 类：

SummaryRanges() 使用一个空数据流初始化对象。
void addNum(int val) 向数据流中加入整数 val 。
int[][] getIntervals() 以不相交区间 [starti, endi] 的列表形式返回对数据流中整数的总结。

https://leetcode.cn/problems/data-stream-as-disjoint-intervals/description/
*/
#[derive(Default)]
struct SummaryRanges {
    intervals: Vec<Vec<i32>>,
}

impl SummaryRanges {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, value: i32) {
        if self.intervals.is_empty() {
            self.intervals.push(vec![value, value]);
            return;
        }
        let idx = self
            .intervals
            .binary_search(&vec![value, value])
            .unwrap_or_else(|x| x);

        let mut merge = false;
        if idx < self.intervals.len() {
            if value + 1 == self.intervals[idx][0] {
                self.intervals[idx][0] -= 1;
                merge = true;
            } else if value == self.intervals[idx][0] {
                merge = true;
            }
        }
        if idx > 0 {
            if value - 1 == self.intervals[idx - 1][1] {
                self.intervals[idx - 1][1] += 1;
                merge = true;
            } else if value <= self.intervals[idx - 1][1] {
                merge = true;
            }
            if idx < self.intervals.len() {
                if self.intervals[idx - 1][1] == self.intervals[idx][0] {
                    self.intervals[idx - 1][1] = self.intervals[idx][1];
                    _ = self.intervals.remove(idx);
                }
            }
        }

        if !merge {
            // 插入
            self.intervals.insert(idx, vec![value, value]);
        }
    }

    fn get_intervals(&mut self) -> Vec<Vec<i32>> {
        return self.intervals.clone();
    }
}

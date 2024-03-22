/**
2671. 频率跟踪器

请你设计并实现一个能够对其中的值进行跟踪的数据结构，并支持对频率相关查询进行应答。

实现 FrequencyTracker 类：

FrequencyTracker()：使用一个空数组初始化 FrequencyTracker 对象。
void add(int number)：添加一个 number 到数据结构中。
void deleteOne(int number)：从数据结构中删除一个 number 。数据结构 可能不包含 number ，在这种情况下不删除任何内容。
bool hasFrequency(int frequency): 如果数据结构中存在出现 frequency 次的数字，则返回 true，否则返回 false。

Hint:
1 <= number <= 105
1 <= frequency <= 105
最多调用 add、deleteOne 和 hasFrequency 共计 2 * 105 次

https://leetcode.cn/problems/frequency-tracker/description/
*/
use std::collections::HashMap;

struct FrequencyTracker {
    fre_hashmap: HashMap<i32, i32>,
    fre_cnt_hashmap: HashMap<i32, i32>,
}

impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker {
            fre_hashmap: HashMap::new(),
            fre_cnt_hashmap: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        let value = *self.fre_hashmap.get(&number).unwrap_or(&0);
        *self.fre_hashmap.entry(number).or_insert(0) += 1;
        *self.fre_cnt_hashmap.entry(value).or_insert(0) -= 1;
        *self.fre_cnt_hashmap.entry(value + 1).or_insert(0) += 1;
    }

    fn delete_one(&mut self, number: i32) {
        let value = *self.fre_hashmap.get(&number).unwrap_or(&0);
        if value > 0 {
            *self.fre_hashmap.entry(number).or_insert(0) -= 1;
            *self.fre_cnt_hashmap.entry(value - 1).or_insert(0) += 1;
            *self.fre_cnt_hashmap.entry(value).or_insert(0) -= 1;
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        return *self.fre_cnt_hashmap.get(&frequency).unwrap_or(&0) > 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let mut s = FrequencyTracker::new();
        s.add(3);
        s.add(3);
        println!(
            "{},{},{},{}",
            s.has_frequency(0),
            s.has_frequency(1),
            s.has_frequency(2),
            s.has_frequency(3)
        );

        assert_eq!(s.has_frequency(2), true);
        s.delete_one(3);
        assert_eq!(s.has_frequency(2), false);
        assert_eq!(s.has_frequency(1), true);
        s.delete_one(3);
        s.delete_one(3);
        assert_eq!(s.has_frequency(0), false);
    }
    #[test]
    fn test_case() {}
}

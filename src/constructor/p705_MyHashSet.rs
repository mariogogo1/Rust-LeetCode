/**
705. 设计哈希集合

不使用任何内建的哈希表库设计一个哈希集合（HashSet）。

实现 MyHashSet 类：

void add(key) 向哈希集合中插入值 key 。
bool contains(key) 返回哈希集合中是否存在这个值 key 。
void remove(key) 将给定值 key 从哈希集合中删除。如果哈希集合中没有这个值，什么也不做。


https://leetcode.cn/problems/design-hashset/description/
*/

struct MyHashSet {
    exists: Vec<u64>,
}

impl MyHashSet {
    fn new() -> Self {
        MyHashSet {
            exists: vec![0; 15626],
        }
    }

    fn add(&mut self, key: i32) {
        self.exists[(key / 64) as usize] &= 1 << (key % 64);
    }

    fn remove(&mut self, key: i32) {
        self.exists[(key / 64) as usize] &= !(1 << (key % 64));
    }

    fn contains(&self, key: i32) -> bool {
        self.exists[(key / 64) as usize] & 1 << (key % 64) > 0
    }
}

/// 這是最最簡單的想法，可以再進步一點用位元運算儲存當作練習
struct MyHashSet_1 {
    exists: Vec<bool>,
}

impl MyHashSet_1 {
    fn new() -> Self {
        MyHashSet_1 {
            exists: vec![false; 1000001],
        }
    }

    fn add(&mut self, key: i32) {
        self.exists[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.exists[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        self.exists[key as usize]
    }
}

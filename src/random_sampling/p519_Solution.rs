/**
519. 随机翻转矩阵

给你一个 m x n 的二元矩阵 matrix ，且所有值被初始化为 0 。请你设计一个算法，随机选取一个满足 matrix[i][j] == 0 的下标 (i, j) ，并将它的值变为 1 。所有满足 matrix[i][j] == 0 的下标 (i, j) 被选取的概率应当均等。

尽量最少调用内置的随机函数，并且优化时间和空间复杂度。

实现 Solution 类：

Solution(int m, int n) 使用二元矩阵的大小 m 和 n 初始化该对象
int[] flip() 返回一个满足 matrix[i][j] == 0 的随机下标 [i, j] ，并将其对应格子中的值变为 1
void reset() 将矩阵中所有的值重置为 0

提示：

1 <= m, n <= 104
每次调用flip 时，矩阵中至少存在一个值为 0 的格子。
最多调用 1000 次 flip 和 reset 方法。

https://leetcode.cn/problems/random-flip-matrix/description/
*/
use rand::Rng;
use std::collections::HashMap;

struct Solution {
    rng: rand::rngs::ThreadRng,
    changes: HashMap<i32, i32>,
    total: i32,
    m: i32,
    n: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Solution {
            rng: rand::thread_rng(),
            changes: HashMap::new(),
            total: m * n,
            m,
            n,
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let s = self.rng.gen_range(1..=self.total);

        let ans = *self.changes.get(&s).unwrap_or(&s);
        let end = *self.changes.get(&self.total).unwrap_or(&self.total);
        self.changes.insert(s, end);

        self.total -= 1;
        //println!("{:?}", self.changes);
        vec![(ans - 1) / self.n, (ans - 1) % self.n]
    }

    fn reset(&mut self) {
        self.changes.clear();
        self.total = self.m * self.n;
    }
}

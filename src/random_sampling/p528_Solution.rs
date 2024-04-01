/**
528. 按权重随机选择

给你一个 下标从 0 开始 的正整数数组 w ，其中 w[i] 代表第 i 个下标的权重。

请你实现一个函数 pickIndex ，它可以 随机地 从范围 [0, w.length - 1] 内（含 0 和 w.length - 1）选出并返回一个下标。选取下标 i 的 概率 为 w[i] / sum(w) 。

例如，对于 w = [1, 3]，挑选下标 0 的概率为 1 / (1 + 3) = 0.25 （即，25%），而选取下标 1 的概率为 3 / (1 + 3) = 0.75（即，75%）。

提示：

1 <= w.length <= 104
1 <= w[i] <= 105
pickIndex 将被调用不超过 104 次

https://leetcode.cn/problems/random-pick-with-weight/description/
*/
use rand::Rng;
struct Solution {
    rng: rand::rngs::ThreadRng,
    weight: Vec<i32>,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let n = w.len();
        let mut solution = Solution {
            rng: rand::thread_rng(),
            weight: vec![0; n + 1],
        };
        let mut sum = 0;
        for (i, v) in w.iter().enumerate() {
            sum += v;
            solution.weight[i + 1] = sum;
        }
        return solution;
    }

    fn pick_index(&mut self) -> i32 {
        let n = self.weight.len();
        let mut prob: i32 = self.rng.gen_range(0..self.weight[n - 1]);
        let mut l: usize = 0;
        let mut r: usize = n - 1;
        while l < r {
            let m = (l + r) / 2;
            if prob >= self.weight[m + 1] {
                l = m + 1;
            } else if prob < self.weight[m] {
                r = m;
            } else {
                l = m;
                break;
            }
        }
        l as i32
    }
}

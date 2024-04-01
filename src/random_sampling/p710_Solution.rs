/**
710. 黑名单中的随机数

给定一个整数 n 和一个 无重复 黑名单整数数组 blacklist 。设计一种算法，从 [0, n - 1] 范围内的任意整数中选取一个 未加入 黑名单 blacklist 的整数。任何在上述范围内且不在黑名单 blacklist 中的整数都应该有 同等的可能性 被返回。

优化你的算法，使它最小化调用语言 内置 随机函数的次数。

实现 Solution 类:

Solution(int n, int[] blacklist) 初始化整数 n 和被加入黑名单 blacklist 的整数
int pick() 返回一个范围为 [0, n - 1] 且不在黑名单 blacklist 中的随机整数

提示：

1 <= n <= 109
0 <= blacklist.length <= min(105, n - 1)
0 <= blacklist[i] < n
blacklist 中所有值都 不同
 pick 最多被调用 2 * 104 次

https://leetcode.cn/problems/random-pick-with-weight/description/
*/
///  不能用記錄白名單的方式因為數字量是10^9過大
///  參考 528按权重随机选择
///  每個黑名單當作下標
///  黑名單與黑名單之間的差-1，就是權重
///  參考528快速依照總權重找到下標(start)，在依照該權重抽取隨機數(add)
///  每次返回 start+add 即可按機率抽取
use rand::Rng;

struct Solution {
    rng: rand::rngs::ThreadRng,
    start: Vec<i32>,
    weights: Vec<i32>,
}

impl Solution {
    fn new(n: i32, mut blacklist: Vec<i32>) -> Self {
        let mut solution = Solution {
            rng: rand::thread_rng(),
            start: Vec::new(),
            weights: Vec::new(),
        };
        blacklist.sort();
        let mut starter = -1;
        let mut sum_weight = 0;
        solution.start.push(starter);
        solution.weights.push(sum_weight);
        for &black in &blacklist {
            if black - starter - 1 > 0 {
                solution.start.push(starter + 1);
                sum_weight += black - starter - 1;
                solution.weights.push(sum_weight);
            }
            starter = black;
        }
        if n - starter - 1 > 0 {
            solution.start.push(starter + 1);
            sum_weight += n - starter - 1;
            solution.weights.push(sum_weight);
        }

        solution
    }

    fn pick(&mut self) -> i32 {
        let n = self.weights.len();
        let prob: i32 = self.rng.gen_range(0..self.weights[n - 1]);
        let mut l: usize = 0;
        let mut r: usize = n - 1;
        while l < r {
            let m = (l + r) / 2;
            if prob >= self.weights[m + 1] {
                l = m + 1;
            } else if prob < self.weights[m] {
                r = m;
            } else {
                l = m;
                break;
            }
        }
        // println!("{:?}",self.weights);
        // println!("{:?}",self.start);
        let add: i32 = self
            .rng
            .gen_range(0..(self.weights[l + 1] - self.weights[l]));

        self.start[l + 1] + add
    }
}

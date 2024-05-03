/**
2462. 雇佣 K 位工人的总代价

给你一个下标从 0 开始的整数数组 costs ，其中 costs[i] 是雇佣第 i 位工人的代价。

同时给你两个整数 k 和 candidates 。我们想根据以下规则恰好雇佣 k 位工人：

总共进行 k 轮雇佣，且每一轮恰好雇佣一位工人。
在每一轮雇佣中，从最前面 candidates 和最后面 candidates 人中选出代价最小的一位工人，如果有多位代价相同且最小的工人，选择下标更小的一位工人。
比方说，costs = [3,2,7,7,1,2] 且 candidates = 2 ，第一轮雇佣中，我们选择第 4 位工人，因为他的代价最小 [3,2,7,7,1,2] 。
第二轮雇佣，我们选择第 1 位工人，因为他们的代价与第 4 位工人一样都是最小代价，而且下标更小，[3,2,7,7,2] 。注意每一轮雇佣后，剩余工人的下标可能会发生变化。
如果剩余员工数目不足 candidates 人，那么下一轮雇佣他们中代价最小的一人，如果有多位代价相同且最小的工人，选择下标更小的一位工人。
一位工人只能被选择一次。
返回雇佣恰好 k 位工人的总代价。

https://leetcode.cn/problems/total-cost-to-hire-k-workers/description/
*/
pub struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut l = (candidates - 1) as usize;
        let mut r = costs.len() - candidates as usize;
        let mut heap = BinaryHeap::new();
        let mut ans = 0 as i64;

        for i in 0..=l {
            heap.push(Reverse((costs[i], i)));
        }
        for i in r.max(l + 1)..costs.len() {
            heap.push(Reverse((costs[i], i)));
        }

        for _ in 0..k {
            if let Some(Reverse(pair)) = heap.pop() {
                ans += pair.0 as i64;
                if l + 1 >= r {
                    continue;
                } else {
                    // 加入新的數字到HEAP
                    if pair.1 <= l {
                        l += 1;
                        heap.push(Reverse((costs[l], l)));
                    } else {
                        r -= 1;
                        heap.push(Reverse((costs[r], r)));
                    }
                }
            }
        }

        ans
    }
}

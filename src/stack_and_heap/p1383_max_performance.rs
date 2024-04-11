/**
1383. 最大的团队表现值

给定两个整数 n 和 k，以及两个长度为 n 的整数数组 speed 和 efficiency。现有 n 名工程师，编号从 1 到 n。其中 speed[i] 和 efficiency[i] 分别代表第 i 位工程师的速度和效率。

从这 n 名工程师中最多选择 k 名不同的工程师，使其组成的团队具有最大的团队表现值。

团队表现值 的定义为：一个团队中「所有工程师速度的和」乘以他们「效率值中的最小值」。

请你返回该团队的​​​​​​最大团队表现值，由于答案可能很大，请你返回结果对 10^9 + 7 取余后的结果。
https://leetcode.cn/problems/maximum-performance-of-a-team/description/
*/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// 照efficiency做遞減排序
/// 挑出K-1個efficiency > 特定元素X的組合，組出以X為最小efficiency的最大團隊表現
/// 遍歷所有X
/// K-1個數字可以維護小根推，把最小的數字踢掉
impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;
    pub fn max_performance(n: i32, mut speed: Vec<i32>, mut efficiency: Vec<i32>, k: i32) -> i32 {
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut ans: i64 = 0;
        let mut sum: i64 = 0;
        let mut min_eff: i64 = i64::MAX;
        let k = k as usize;
        let n = n as usize;
        let mut pair: Vec<(&i32, &i32)> = efficiency.iter().zip(speed.iter()).collect();

        pair.sort_by(|a, b| b.0.cmp(&a.0));
        // println!("{:?}", pair);

        for i in 0..n {
            min_eff = *pair[i].0 as i64;
            if min_heap.len() >= k {
                if let Some(&Reverse(value)) = min_heap.peek() {
                    if value < *pair[i].1 {
                        min_heap.pop();
                        min_heap.push(Reverse(*pair[i].1));
                        sum -= value as i64;
                        sum += *pair[i].1 as i64;
                    }
                }
            } else {
                min_heap.push(Reverse(*pair[i].1));
                sum += *pair[i].1 as i64;
            }
            ans = ans.max(min_eff * sum);

            //      println!("{}", min_eff);
            //    println!("{}", ans);
        }
        (ans % Self::VAL_MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2),
            60
        );
        assert_eq!(
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 4),
            72
        );
    }
    #[test]
    fn test_case() {
        assert_eq!(
            Solution::max_performance(3, vec![2, 8, 2], vec![2, 7, 1], 2),
            56
        );
    }
}

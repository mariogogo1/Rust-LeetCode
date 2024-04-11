/**
3075. 幸福值最大化的选择方案

给你一个长度为 n 的数组 happiness ，以及一个 正整数 k 。

n 个孩子站成一队，其中第 i 个孩子的 幸福值 是 happiness[i] 。你计划组织 k 轮筛选从这 n 个孩子中选出 k 个孩子。

在每一轮选择一个孩子时，所有 尚未 被选中的孩子的 幸福值 将减少 1 。注意，幸福值 不能 变成负数，且只有在它是正数的情况下才会减少。

选择 k 个孩子，并使你选中的孩子幸福值之和最大，返回你能够得到的 最大值 。

https://leetcode.cn/problems/maximize-happiness-of-selected-children/description/
*/

pub struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    // 排序
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort_unstable(); //相等元素的相對位置可能被改變

        let n = happiness.len();
        let mut ans: i64 = 0;
        for i in 0..k {
            if happiness[n - 1 - i as usize] > i {
                ans += (happiness[n - 1 - i as usize] - i) as i64;
            } else {
                break;
            }
        }

        ans
    }

    // 練習 HEAP
    pub fn maximum_happiness_sum_heap(happiness: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for i in 0..k {
            min_heap.push(Reverse(happiness[i]));
        }

        for i in k..happiness.len() {
            if let Some(&Reverse(value)) = min_heap.peek() {
                if value < happiness[i] {
                    min_heap.pop();
                    min_heap.push(Reverse(happiness[i]));
                }
            }
        }

        let mut ans: i64 = 0;
        for i in (0..k).rev() {
            if let Some(Reverse(value)) = min_heap.pop() {
                if value > i as i32 {
                    ans += value as i64 - i as i64;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::maximum_happiness_sum(vec![1, 2, 3], 2), 4);
        assert_eq!(Solution::maximum_happiness_sum(vec![1, 1, 1, 1], 2), 1);
    }
    #[test]
    fn test_case() {}
}

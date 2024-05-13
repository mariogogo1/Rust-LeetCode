/**
1705. 吃苹果的最大数目

有一棵特殊的苹果树，一连 n 天，每天都可以长出若干个苹果。在第 i 天，树上会长出 apples[i] 个苹果，这些苹果将会在 days[i] 天后（也就是说，第 i + days[i] 天时）腐烂，变得无法食用。也可能有那么几天，树上不会长出新的苹果，此时用 apples[i] == 0 且 days[i] == 0 表示。

你打算每天 最多 吃一个苹果来保证营养均衡。注意，你可以在这 n 天之后继续吃苹果。

给你两个长度为 n 的整数数组 days 和 apples ，返回你可以吃掉的苹果的最大数目。

https://leetcode.cn/problems/maximum-number-of-eaten-apples/description/
*/

pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// 吃掉離保鮮期剩餘時間最短的蘋果,如果已經爛掉就踢出最小堆
impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut min_time_heap = BinaryHeap::new();
        let mut ans = 0;
        let n = apples.len() as usize;

        for i in 0..n {
            min_time_heap.push(Reverse((i as i32 + days[i], apples[i])));
            while let Some(Reverse((deadline, count))) = min_time_heap.pop() {
                if deadline > i as i32 {
                    if count > 1 {
                        min_time_heap.push(Reverse((deadline, count - 1)));
                    }
                    ans += 1;
                    break;
                }
            }
        }

        let mut day = n as i32;
        while !min_time_heap.is_empty() {
            while let Some(Reverse((deadline, count))) = min_time_heap.pop() {
                if deadline > day {
                    if count > 1 {
                        min_time_heap.push(Reverse((deadline, count - 1)));
                    }
                    ans += 1;
                    break;
                }
            }
            day += 1;
        }

        ans
    }
}

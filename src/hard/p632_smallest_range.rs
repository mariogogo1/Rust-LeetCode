/**
632. 最小区间

你有 k 个 非递减排列 的整数列表。找到一个 最小 区间，使得 k 个列表中的每个列表至少有一个数包含在其中。

我们定义如果 b-a < d-c 或者在 b-a == d-c 时 a < c，则区间 [a,b] 比 [c,d] 小。

https://leetcode.cn/problems/smallest-range-covering-elements-from-k-lists/description/
*/

pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let k = nums.len();
        let mut pair = Vec::new();
        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                pair.push((nums[i][j], i));
            }
        }

        pair.sort_unstable_by_key(|&x| x.0); // 按照元组的第一个元素进行排序

        let mut hash = HashMap::new();
        let mut r = 0;
        let mut l = 0;
        let mut ans: Vec<i32> = vec![0, i32::MAX / 2];
        loop {
            *hash.entry(pair[r].1).or_insert(0) += 1;
            if hash.len() == k {
                if pair[r].0 + ans[0] < pair[l].0 + ans[1] {
                    ans = vec![pair[l].0, pair[r].0];
                }
            }

            loop {
                if hash.len() < k {
                    break;
                }
                let mut val = hash.entry(pair[l].1).or_insert(0);
                *val -= 1;
                if *val == 0 {
                    hash.remove(&pair[l].1);
                }

                l += 1;
                if hash.len() == k {
                    if pair[r].0 + ans[0] < pair[l].0 + ans[1] {
                        ans = vec![pair[l].0, pair[r].0];
                    }
                }
            }

            r += 1;
            if r == pair.len() {
                break;
            }
        }

        ans
    }
}

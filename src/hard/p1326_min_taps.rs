/**
1326. 灌溉花园的最少水龙头数目

在 x 轴上有一个一维的花园。花园长度为 n，从点 0 开始，到点 n 结束。

花园里总共有 n + 1 个水龙头，分别位于 [0, 1, ..., n] 。

给你一个整数 n 和一个长度为 n + 1 的整数数组 ranges ，其中 ranges[i] （下标从 0 开始）表示：如果打开点 i 处的水龙头，可以灌溉的区域为 [i -  ranges[i], i + ranges[i]] 。

请你返回可以灌溉整个花园的 最少水龙头数目 。如果花园始终存在无法灌溉到的地方，请你返回 -1 。

https://leetcode.cn/problems/minimum-number-of-taps-to-open-to-water-a-garden/description/
*/
pub struct Solution;

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut intervals: Vec<(i32, i32)> = Vec::new();
        for i in 0..=n {
            let min_v = 0.max(i - ranges[i as usize]);
            let max_v = (n).min(i + ranges[i as usize]);

            intervals.push((min_v, max_v))
        }

        let mut ans: Vec<(i32, i32)> = Vec::new();
        for &interval in intervals.iter() {
            while let Some(old_interval) = ans.last() {
                if old_interval.1 <= interval.1 {
                    if old_interval.0 >= interval.0 {
                        ans.pop();
                    } else {
                        if ans.len() >= 2 && ans[ans.len() - 2].1 >= interval.0 {
                            ans.pop();
                        } else if old_interval.1 >= interval.0 {
                            if old_interval.1 < interval.1 {
                                ans.push(interval);
                            }
                            break;
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }

            if ans.is_empty() {
                ans.push(interval);
            }
        }

        if ans[ans.len() - 1].1 < n {
            return -1;
        }

        ans.len() as i32
    }
}

/**
1388. 3n 块披萨

给你一个披萨，它由 3n 块不同大小的部分组成，现在你和你的朋友们需要按照如下规则来分披萨：

你挑选 任意 一块披萨。
Alice 将会挑选你所选择的披萨逆时针方向的下一块披萨。
Bob 将会挑选你所选择的披萨顺时针方向的下一块披萨。
重复上述过程直到没有披萨剩下。
每一块披萨的大小按顺时针方向由循环数组 slices 表示。

请你返回你可以获得的披萨大小总和的最大值。

https://leetcode.cn/problems/pizza-with-3n-slices/description/
*/

pub struct Solution;
use std::collections::BinaryHeap;

impl Solution {
    /// 反悔選擇+HEAP
    /// 反悔選擇：
    /// 選擇了 slices[IDX]的結果，同時slices[IDX+1]跟slices[IDX-1]被消耗掉了
    /// 如果在第二次選擇的時候發現 slices[IDX] +OTHER 其實比 slices[IDX+1]+slices[IDX-1] 還要小，這時想要反悔
    /// 可以構建一個新的數字 NEXT="slices[IDX+1]+slices[IDX-1]-slices[IDX]"
    /// NEXT + slices[IDX] 就可以反悔第一次的選擇，改選成為旁邊兩個PIZZA
    pub fn max_size_slices(mut slices: Vec<i32>) -> i32 {
        let n = slices.len();

        let mut max_heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        for i in 0..n {
            max_heap.push((slices[i], i));
        }

        // 構造一個類似雙練表構造並記錄有沒有訪問過
        let mut right: Vec<usize> = (1..n).chain(std::iter::once(0)).collect();
        let mut left: Vec<usize> = std::iter::once(n - 1).chain(0..(n - 1)).collect();
        let mut visited: Vec<bool> = vec![false; n];

        let mut ans = 0;
        let mut k = slices.len() / 3;

        while k > 0 {
            if let Some((value, idx)) = max_heap.pop() {
                if visited[idx] {
                    continue;
                }
                ans += value;
                visited[left[idx]] = true;
                visited[right[idx]] = true;
                slices[idx] = slices[left[idx]] + slices[right[idx]] - slices[idx];
                max_heap.push((slices[idx], idx));
                left[idx] = left[left[idx]];
                right[idx] = right[right[idx]];
                right[left[idx]] = idx;
                left[right[idx]] = idx;

                k -= 1;
            }
        }

        ans
    }

    /// DP 作法
    /// dp[i][j] 拿取第J個PIZZA得到的PIZZA總量
    /// PIZZA不可取得相鄰的，且PIZZA是一個環形數組
    /// 參考213  https://leetcode.cn/problems/house-robber-ii/description/

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::max_size_slices(vec![1, 2, 3, 4, 5, 6]), 10);
        assert_eq!(Solution::max_size_slices(vec![8, 9, 8, 6, 1, 1]), 16);
    }
    #[test]
    fn test_case() {
        assert_eq!(
            Solution::max_size_slices(vec![3, 5, 4, 4, 6, 6, 3, 4, 4, 7, 10, 5, 7, 2, 2]),
            32
        );
    }
}

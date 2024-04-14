/**
1696. 跳跃游戏 VI

给你一个下标从 0 开始的整数数组 nums 和一个整数 k 。

一开始你在下标 0 处。每一步，你最多可以往前跳 k 步，但你不能跳出数组的边界。也就是说，你可以从下标 i 跳到 [i + 1， min(n - 1, i + k)] 包含 两个端点的任意位置。

你的目标是到达数组最后一个位置（下标为 n - 1 ），你的 得分 为经过的所有数字之和。

请你返回你能得到的 最大得分 。

https://leetcode.cn/problems/jump-game-vi/description/
*/
use std::collections::BinaryHeap;
//use std::collections::HashMap;
impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();
        let mut dp = vec![0; n];
        dp[0] = nums[0];
        let mut max_v = nums[0];
        let mut key = 0;

        for i in 1..n {
            // 維護指針，因為做DP每次往前K個項目找最大值，其中有K-1項目在前一個數字已經都查過了，只要確定最新加入的數字不是最大值，或是最大值剛剛離開K的範圍
            // 那麼最大值的位置就不會改變
            if key == i - k - 1 {
                max_v = i32::MIN;
                for j in 1..=k {
                    if i < j {
                        break;
                    }
                    if nums[i - j] >= 0 {
                        max_v = dp[i - j];
                        key = i - j;
                        break;
                    }
                    if dp[i - j] > max_v {
                        max_v = dp[i - j];
                        key = i - j;
                    }
                }
            } else {
                if dp[i - 1] > max_v {
                    max_v = dp[i - 1];
                    key = i - 1;
                }
            }

            dp[i] = dp[key] + nums[i];
        }

        dp[n - 1]
    }

    // heap
    pub fn max_result_heap(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n];
        let mut max_heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();

        dp[0] = nums[0];
        max_heap.push((dp[0], 0));

        for i in 1..n {
            while let Some(&val) = max_heap.peek() {
                if val.1 + (k as usize) < i {
                    max_heap.pop();
                } else {
                    dp[i] = nums[i] + val.0;
                    max_heap.push((dp[i], i));
                    break;
                }
            }
        }

        dp[n - 1]
    }
}

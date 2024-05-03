/**
698. 划分为k个相等的子集

给定一个整数数组  nums 和一个正整数 k，找出是否有可能把这个数组分成 k 个非空子集，其总和都相等。

https://leetcode.cn/problems/partition-to-k-equal-sum-subsets/description/
*/

// 用DFS的方式會再快一點
pub struct Solution;
impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        let mut sum = 0;
        for &num in nums.iter() {
            sum += num;
        }
        if sum % k != 0 {
            return false;
        }

        nums.sort_unstable();
        let x = sum / k;

        if *nums.last().unwrap() > x {
            return false;
        }

        let n = nums.len();

        let mut dp = vec![false; (1 << n) as usize];
        let mut cur_sum_dp = vec![0; (1 << n) as usize];

        dp[0] = true;
        for i in 0..dp.len() {
            if dp[i] {
                for j in 0..nums.len() {
                    if cur_sum_dp[i] + nums[j] > x {
                        break;
                    }
                    if i >> j & 1 == 1 {
                        continue;
                    }
                    let new_sum = (cur_sum_dp[i] + nums[j]) % x;
                    let new_state = 1 << j | i;
                    if dp[new_state] {
                        continue;
                    }
                    cur_sum_dp[new_state] = new_sum;
                    dp[new_state] = true;
                }
            }
        }
        dp[dp.len() - 1]
    }
}

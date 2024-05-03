/**
416. 分割等和子集
给你一个 只包含正整数 的 非空 数组 nums 。请你判断是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。
https://leetcode.cn/problems/partition-equal-subset-sum/description/
*/
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let interval = (sum / 2) as usize;
        let mut dp = vec![false; interval + 1];
        dp[0] = true;

        for &num in nums.iter() {
            for i in (0..=interval).rev() {
                if dp[i] {
                    let new_sum = i as i32 + num;
                    if new_sum < sum / 2 {
                        dp[new_sum as usize] = true;
                    } else if new_sum == sum / 2 {
                        return true;
                    }
                }
            }
        }
        dp[interval]
    }
}

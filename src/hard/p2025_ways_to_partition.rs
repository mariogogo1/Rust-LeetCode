/**

2025. 分割数组的最多方案数

给你一个下标从 0 开始且长度为 n 的整数数组 nums 。分割 数组 nums 的方案数定义为符合以下两个条件的 pivot 数目：

1 <= pivot < n
nums[0] + nums[1] + ... + nums[pivot - 1] == nums[pivot] + nums[pivot + 1] + ... + nums[n - 1]
同时给你一个整数 k 。你可以将 nums 中 一个 元素变为 k 或 不改变 数组。

请你返回在 至多 改变一个元素的前提下，最多 有多少种方法 分割 nums 使得上述两个条件都满足。

https://leetcode.cn/problems/maximum-number-of-ways-to-partition-an-array/description/
*/
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut ans = 0;

        let mut diff_hashmap: HashMap<i64, i32> = HashMap::new();

        let mut sum = nums.iter().fold(0, |s, v| s + (*v) as i64);

        let mut another_sum = 0;
        for i in 1..n {
            another_sum += nums[i - 1] as i64;
            *diff_hashmap.entry(sum - 2 * another_sum).or_insert(0) += 1;
        }

        ans = *diff_hashmap.get(&0).unwrap_or(&0);

        for i in 0..n {
            let x = (k - nums[i]) as i64;
            let ans_a = *diff_hashmap.get(&x).unwrap_or(&0);
            ans = ans.max(ans_a);
            sum -= 2 * nums[i] as i64;
            *diff_hashmap.entry(sum).or_insert(0) -= 1;
            *diff_hashmap.entry(-sum).or_insert(0) += 1;
        }

        ans
    }
}

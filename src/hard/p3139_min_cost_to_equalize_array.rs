/**

3139. 使数组中所有元素相等的最小开销

给你一个整数数组 nums 和两个整数 cost1 和 cost2 。你可以执行以下 任一 操作 任意 次：

从 nums 中选择下标 i 并且将 nums[i] 增加 1 ，开销为 cost1。
选择 nums 中两个 不同 下标 i 和 j ，并且将 nums[1] 和 nums[2] 都 增加 1 ，开销为 cost2 。
你的目标是使数组中所有元素都 相等 ，请你返回需要的 最小开销 之和。

由于答案可能会很大，请你将它对 109 + 7 取余 后返回。

https://leetcode.cn/problems/minimum-cost-to-equalize-array/description/
*/
pub struct Solution;

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn min_cost_to_equalize_array(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
        let mut sum = 0 as i64;
        let mut max_v = 0 as i32;
        let n = nums.len() as i64;
        let cost1 = cost1 as i64;
        let cost2 = cost2 as i64;

        for &num in nums.iter() {
            sum += num as i64;
            max_v = max_v.max(num);
        }
        if cost1 * 2 <= cost2 {
            return (((max_v as i64 * n - sum) * cost1) % Self::VAL_MOD) as i32;
        }

        let mut left_vec = Vec::new();

        for &num in nums.iter() {
            if max_v > num {
                left_vec.push(max_v - num);
            }
        }

        let mut ans = 0 as i64;
        let length = n - 1;
        let mut left = 0 as i64;
        let mut total_pairs = 0 as i64;
        for i in 0..left_vec.len() {
            let num = left_vec[i] as i64;
            if left >= num {
                left -= num;
                total_pairs += num;
            } else {
                let s = total_pairs * 2 + left;
                if s <= num {
                    total_pairs = s;
                    left = num - s;
                } else {
                    total_pairs += (left + num) / 2;
                    left = (left + num) % 2;
                }
            }
        }
        ans = (total_pairs * cost2) % Self::VAL_MOD;

        if cost2 * length < cost1 * (length - 1) {
            if left >= length - 1 {
                let cal = left / (length - 1);
                left = left - cal * (length - 1);
                ans += (cal * length * cost2) % Self::VAL_MOD;
            }

            if length > 1 {
                if n % 2 == 0 && left % 2 == 1 {
                    if (n + left) / 2 * cost2 < cost1 * (left - 1) {
                        ans += (n + left) / 2 * cost2;
                        left = 1;
                    }
                } else if n % 2 == 1 && left % 2 == 0 {
                    if n + left / 2 * cost2 < cost1 * left {
                        ans += (n + left / 2) * cost2;
                        left = 0;
                    }
                } else {
                    if (n + left) / 2 * cost2 < cost1 * left {
                        ans += (n + left) / 2 * cost2;
                        left = 0;
                    }
                }
            }
        }

        ans += left * cost1;

        (ans % Self::VAL_MOD) as i32
    }
}

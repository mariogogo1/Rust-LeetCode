/**
16. 最接近的三数之和

给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。

返回这三个数的和。

假定每组输入只存在恰好一个解。

https://leetcode.cn/problems/3sum-closest/description/
*/
pub struct Solution;
impl Solution {
    fn abs(x: i32) -> i32 {
        if x < 0 {
            return -x;
        }
        x
    }

    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut ans = nums[0] + nums[1] + nums[nums.len() - 1];

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = nums.len() - 1;

            while l < r {
                let sum = nums[i] + nums[l] + nums[r];

                if Self::abs(sum - target) < Self::abs(ans - target) {
                    ans = sum;
                }

                if sum < target {
                    l += 1;
                } else if sum > target {
                    r -= 1;
                } else {
                    return target;
                }
            }
        }

        ans
    }
}

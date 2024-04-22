/**
18. 四数之和

给你一个由 n 个整数组成的数组 nums ，和一个目标值 target 。请你找出并返回满足下述全部条件且不重复的四元组 [nums[a], nums[b], nums[c], nums[d]] （若两个四元组元素一一对应，则认为两个四元组重复）：

0 <= a, b, c, d < n
a、b、c 和 d 互不相同
nums[a] + nums[b] + nums[c] + nums[d] == target
你可以按 任意顺序 返回答案 。
https://leetcode.cn/problems/4sum/description/
*/
pub struct Solution;
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let target = target as i64;

        nums.sort_unstable();
        let mut ans = Vec::new();

        if nums.len() < 4 {
            return ans;
        }

        for i in 0..(nums.len() - 3) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            for j in ((i + 3)..nums.len()).rev() {
                if j < nums.len() - 1 && nums[j] == nums[j + 1] {
                    continue;
                }

                let mut l = i + 1;
                let mut r = j - 1;

                while l < r {
                    let sum = nums[i] as i64 + nums[l] as i64 + nums[r] as i64 + nums[j] as i64;

                    if sum == target {
                        ans.push(vec![nums[i], nums[l], nums[r], nums[j]]);
                        // Skip duplicates
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        l += 1;
                        r -= 1;
                    } else if sum < target {
                        l += 1;
                    } else {
                        r -= 1;
                    }
                }
            }
        }

        ans
    }
}

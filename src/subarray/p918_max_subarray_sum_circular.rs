/**
918. 环形子数组的最大和

给定一个长度为 n 的环形整数数组 nums ，返回 nums 的非空 子数组 的最大可能和 。

环形数组 意味着数组的末端将会与开头相连呈环状。形式上， nums[i] 的下一个元素是 nums[(i + 1) % n] ， nums[i] 的前一个元素是 nums[(i - 1 + n) % n] 。

子数组 最多只能包含固定缓冲区 nums 中的每个元素一次。形式上，对于子数组 nums[i], nums[i + 1], ..., nums[j] ，不存在 i <= k1, k2 <= j 其中 k1 % n == k2 % n 。


https://leetcode.cn/problems/maximum-sum-circular-subarray/description/
*/
pub struct Solution;
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut ans_max: i32 = i32::MIN;
        let mut current_max: i32 = 0;
        let mut ans_min: i32 = i32::MAX;
        let mut current_min: i32 = 0;
        let mut sum: i32 = 0;

        for num in nums {
            sum += num;
            current_max = current_max.max(0) + num;
            ans_max = ans_max.max(current_max);
            current_min = current_min.min(0) + num;
            ans_min = ans_min.min(current_min);
        }
        // 特例處理 環形 當sum == ans_min此情況表示子數組為空集合，要否定掉
        if sum == ans_min {
            return ans_max;
        }
        return ans_max.max(sum - ans_min);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
        assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
        assert_eq!(Solution::max_subarray_sum_circular(vec![-3, -2, -3]), -2);
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

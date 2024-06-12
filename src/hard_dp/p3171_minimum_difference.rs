/**
3171. 找到按位与最接近 K 的子数组

给你一个数组 nums 和一个整数 k 。你需要找到 nums 的一个
子数组
 ，满足子数组中所有元素按位与运算 AND 的值与 k 的 绝对差 尽可能 小 。换言之，你需要选择一个子数组 nums[l..r] 满足 |k - (nums[l] AND nums[l + 1] ... AND nums[r])| 最小。

请你返回 最小 的绝对差值。

子数组是数组中连续的 非空 元素序列。

https://leetcode.cn/problems/find-subarray-with-bitwise-and-closest-to-k/description/
*/
pub struct Solution;
// 數字大小為10^9 最多是2進制只有30位數字，所以第二層迴圈最多30次->時間複雜度就"不會是O(N^2)"
// 同樣想法可以延伸題目為 "或運算" "GCD" "LCM"
impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = i32::MAX;
        for i in 0..nums.len() {
            let mut x = nums[i];
            if k - x >= 0 {
                ans = ans.min(k - x);
            } else {
                ans = ans.min(x - k);
            }
            for j in (0..i).rev() {
                if x & nums[j] == nums[j] {
                    break;
                }
                nums[j] &= x;
                if k - nums[j] >= 0 {
                    ans = ans.min(k - nums[j]);
                } else {
                    ans = ans.min(nums[j] - k);
                }
            }
        }
        ans
    }
}

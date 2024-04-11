/**

891. 子序列宽度之和

一个序列的 宽度 定义为该序列中最大元素和最小元素的差值。

给你一个整数数组 nums ，返回 nums 的所有非空 子序列 的 宽度之和 。由于答案可能非常大，请返回对 109 + 7 取余 后的结果。

子序列 定义为从一个数组里删除一些（或者不删除）元素，但不改变剩下元素的顺序得到的数组。例如，[3,6,2,7] 就是数组 [0,3,1,6,2,2,7] 的一个子序列。

https://leetcode.cn/problems/sum-of-subsequence-widths/description/
*/
pub struct Solution;

/// 排序成為遞增數列
/// n=nums.len();
/// 比如第1個數字x 擁有第一個數字x的子序列一共有2^(n-1)種，所以用類似前綴和的方式累計x*2^(n-1)。
/// 最後再用最大值的前綴和-最小值的前綴和 = ans
impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut plus_sum: i64 = 0;
        let mut minus_sum: i64 = 0;
        for i in 0..nums.len() {
            plus_sum *= 2;
            plus_sum += nums[nums.len() - 1 - i] as i64;
            plus_sum %= Self::VAL_MOD;

            minus_sum *= 2;
            minus_sum += nums[i] as i64;
            minus_sum %= Self::VAL_MOD;
        }
        let mut ans = (plus_sum - minus_sum) % Self::VAL_MOD;
        if ans < 0 {
            ans += Self::VAL_MOD;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case() {}
}

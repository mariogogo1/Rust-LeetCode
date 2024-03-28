/**

2681. 英雄的力量

给你一个下标从 0 开始的整数数组 nums ，它表示英雄的能力值。如果我们选出一部分英雄，这组英雄的 力量 定义为：

i0 ，i1 ，... ik 表示这组英雄在数组中的下标。那么这组英雄的力量为 max(nums[i0],nums[i1] ... nums[ik])2 * min(nums[i0],nums[i1] ... nums[ik]) 。
请你返回所有可能的 非空 英雄组的 力量 之和。由于答案可能非常大，请你将结果对 109 + 7 取余。

提示：

1 <= nums.length <= 105
1 <= nums[i] <= 109

https://leetcode.cn/problems/rearranging-fruits/description/
*/
pub struct Solution;
///
/// n = nums.len();
/// 前綴和使用：
/// 總共的子集數量為(2^n)-1，最小數字min會在其中的2^(n-1)個子集裡面，利用這點推理=>
/// 排序過後每個數字num[i]，對下標比i小的元素num[i](i前面的元素)構成的子集來說都是最大值
/// 所以前面一共有 (2^i)-1 個子集的最大值是num[i]，其中num[0]為最小值佔其中2^(i-1)個子集，
/// 其中num[1]為最小值佔其中2^(i-2)個子集，以此類推.....
/// 用前綴和紀錄子集的最小值之和。
///     

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;
    pub fn sum_of_power(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        //println!("{:?}", nums);
        let mut prefix_sum: i64 = 0;
        let mut ans: i64 = 0;
        for num in nums {
            let num_i64: i64 = num as i64;
            ans += ((num_i64 * num_i64) % Self::VAL_MOD) * (num_i64 + prefix_sum);
            prefix_sum = (prefix_sum * 2 + num_i64) % Self::VAL_MOD;
            ans %= Self::VAL_MOD;
        }

        while ans < 0 {
            ans += Self::VAL_MOD;
        }

        return ans as i32;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::sum_of_power(vec![2, 1, 4]), 141);
        assert_eq!(Solution::sum_of_power(vec![1, 1, 1]), 7);
    }
    #[test]
    fn test_case() {}
}

/**

2897. 对数组执行操作使平方和最大

给你一个下标从 0 开始的整数数组 nums 和一个 正 整数 k 。

你可以对数组执行以下操作 任意次 ：

选择两个互不相同的下标 i 和 j ，同时 将 nums[i] 更新为 (nums[i] AND nums[j]) 且将 nums[j] 更新为 (nums[i] OR nums[j]) ，OR 表示按位 或 运算，AND 表示按位 与 运算。
你需要从最终的数组里选择 k 个元素，并计算它们的 平方 之和。

请你返回你可以得到的 最大 平方和。

由于答案可能会很大，将答案对 109 + 7 取余 后返回。

提示：

1 <= k <= nums.length <= 105
1 <= nums[i] <= 109

https://leetcode.cn/problems/apply-operations-on-array-to-maximize-sum-of-squares/description/
*/

struct Solution {}

/// num[i]=A and  num[j]=B 進行 OR 跟 AND 運算前後的位元符號(1的總數量不會減少)，
/// 且運算操作後 新的兩個數字 C and D 會有以下性質
/// A + B = C + D 且 C >= Max(A,B) 且  D <= Min(A,B)
/// 不失一般性：令 C= A + X  ， D = B - X
/// C^2 + D^2 = A^2 + B^2 + 2*X^2  >= A^2 + B^2
/// 所以可以不斷的做操作 讓C跟D的平方和越來越大
///  
/// 1.計數nums 中 num 的每個位元為1的總數量
/// 2.依照位元數量組合成新的數組，每個數盡量大，該位元有足夠的1使用就用。
/// 3.最大的K個數的平方和

/// 時間複雜度 O(N*log2(max(nums)))，不確定

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn max_sum(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut digits: Vec<i64> = vec![0; 32];
        let mut s: i64 = 0;
        for mut num in nums {
            for i in 0..32 {
                s = (num & 1) as i64;
                digits[i] += s;
                num = num >> 1;
            }
        }
        let mut ans: i64 = 0;

        //println!("{:?}", digits);
        let mut conti = true;
        while k > 0 && conti {
            s = 0;
            for i in (0..32).rev() {
                s = s << 1;
                if digits[i] > 0 {
                    digits[i] -= 1;
                    s += 1;
                }
            }
            if s == 0 {
                conti = false;
            }
            s %= Self::VAL_MOD;
            ans = (ans + s * s) % Self::VAL_MOD;
            k -= 1;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::max_sum(vec![2, 6, 5, 8], 2), 261);
        assert_eq!(Solution::max_sum(vec![4, 5, 4, 7], 3), 90);
    }
    #[test]
    fn test_case_1() {}
    #[test]
    fn test_case_2() {}
}

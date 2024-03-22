/**
1969. 数组元素的最小非零乘积

给你一个正整数 p 。你有一个下标从 1 开始的数组 nums ，这个数组包含范围 [1, 2p - 1] 内所有整数的二进制形式（两端都 包含）。你可以进行以下操作 任意 次：

从 nums 中选择两个元素 x 和 y  。
选择 x 中的一位与 y 对应位置的位交换。对应位置指的是两个整数 相同位置 的二进制位。
比方说，如果 x = 1101 且 y = 0011 ，交换右边数起第 2 位后，我们得到 x = 1111 和 y = 0001 。

请你算出进行以上操作 任意次 以后，nums 能得到的 最小非零 乘积。将乘积对 109 + 7 取余 后返回。

注意：答案应为取余 之前 的最小值。

https://leetcode.cn/problems/powx-n/description/
*/

pub struct Solution;

impl Solution {
    fn quick_pow(x: i64, n: i64) -> i64 {
        if n == 0 {
            return 1;
        }
        let xx = Self::quick_pow((x * x) % _VAL_MOD, n / 2);
        if n % 2 == 1 {
            return (xx * x) % _VAL_MOD;
        }
        return (xx) % _VAL_MOD;
    }

    pub fn min_non_zero_product(p: i32) -> i32 {
        // (2^p-1)*(2^p-2)^(2^(p-1)-1)
        let p_i64 = p as i64;
        let s = Self::quick_pow(2, p_i64) - 1;
        let power = 1 << p - 1;
        let mut ans: i64 = s * Self::quick_pow(s - 1, power - 1);
        return (ans % _VAL_MOD) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::min_non_zero_product(1), 1);
        assert_eq!(Solution::min_non_zero_product(2), 6);
        assert_eq!(Solution::min_non_zero_product(3), 1512);
        assert_eq!(Solution::min_non_zero_product(30), 945196305);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::min_non_zero_product(59), 926891661);
        assert_eq!(Solution::min_non_zero_product(60), 813987236);
    }
}

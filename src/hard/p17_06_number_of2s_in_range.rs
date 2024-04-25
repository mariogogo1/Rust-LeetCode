/**
面试题 17.06. 2出现的次数

编写一个方法，计算从 0 到 n (含 n) 中数字 2 出现的次数。

https://leetcode.cn/problems/number-of-2s-in-range-lcci/description/
*/
pub struct Solution;

/// 類似233.
impl Solution {
    pub fn number_of2s_in_range(mut n: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut digit_count = 0 as usize;
        while n > 0 {
            let mut x = n % 10;
            n /= 10;

            let mut z = 1;
            for _ in 0..digit_count {
                z *= 10;
            }

            if x >= 3 {
                ans += z;
            } else if x == 2 {
                ans += left + 1;
            }
            ans += n * z;

            left += x * z;
            digit_count += 1;
        }

        ans
    }
}

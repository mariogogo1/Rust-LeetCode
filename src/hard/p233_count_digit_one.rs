/**
233. 数字 1 的个数

给定一个整数 n，计算所有小于等于 n 的非负整数中数字 1 出现的个数。

https://leetcode.cn/problems/number-of-digit-one/description/
*/
pub struct Solution;

impl Solution {
    pub fn count_digit_one(mut n: i32) -> i32 {
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

            if x >= 2 {
                ans += z;
            } else if x == 1 {
                ans += left + 1;
            }
            ans += n * z;

            left += x * z;
            digit_count += 1;
        }

        ans
    }
}

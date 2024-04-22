/**
479. 最大回文数乘积

给定一个整数 n ，返回 可表示为两个 n 位整数乘积的 最大回文整数 。因为答案可能非常大，所以返回它对 1337 取余 。

https://leetcode.cn/problems/largest-palindrome-product/description/
*/
pub struct Solution;

// 從10^(2*n) -1 開始找往下找
impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9;
        }
        let mut left: i64 = 1;
        let mut tens: i64 = 1;
        for _ in 0..n {
            left *= 10;
            tens *= 10;
        }
        left -= 1;
        for mut x in (0..=left).rev() {
            let s = x;
            let mut num = x;
            while x > 0 {
                num *= 10;
                num += x % 10;
                x /= 10;
            }
            // println!("{}",num);
            for factor in s..tens {
                if factor * factor > num {
                    break;
                }
                if num % factor == 0 && (num / factor) / tens == 0 {
                    return (num % 1337) as i32;
                }
            }
        }
        0
    }
}

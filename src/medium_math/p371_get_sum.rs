/**
371. 两整数之和

给你两个整数 a 和 b ，不使用 运算符 + 和 - ​​​​​​​，计算并返回两整数之和。

https://leetcode.cn/problems/sum-of-two-integers/description/
*/

pub struct Solution;
impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        let mut next_one = false;
        let mut ans = 0;
        for i in 0..32 {
            let a_digit = a & 1;
            let b_digit = b & 1;
            let mut ans_digit = 0;
            if !next_one {
                match (a_digit, b_digit) {
                    (1, 1) => {
                        next_one = true;
                        ans_digit = 0
                    }
                    (0, 0) => {
                        next_one = false;
                        ans_digit = 0
                    }
                    _ => {
                        next_one = false;
                        ans_digit = 1
                    }
                }
            } else {
                match (a_digit, b_digit) {
                    (1, 1) => {
                        next_one = true;
                        ans_digit = 1
                    }
                    (0, 0) => {
                        next_one = false;
                        ans_digit = 1
                    }
                    _ => {
                        next_one = true;
                        ans_digit = 0
                    }
                }
            }
            ans |= ans_digit << i;

            a >>= 1;
            b >>= 1;
        }
        ans
    }
}

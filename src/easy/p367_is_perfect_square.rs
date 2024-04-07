/**
367. 有效的完全平方数

给你一个正整数 num 。如果 num 是一个完全平方数，则返回 true ，否则返回 false 。

完全平方数 是一个可以写成某个整数的平方的整数。换句话说，它可以写成某个整数和自身的乘积。

不能使用任何内置的库函数，如  sqrt 。

https://leetcode.cn/problems/valid-perfect-square/description/
*/

pub struct Solution;
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let mut l: i64 = 0;
        let mut r: i64 = num as i64;
        while l < r {
            let m = (l + r) / 2;
            if m * m == num {
                return true;
            } else if m * m > num {
                r = m;
            } else {
                l = m + 1;
            }
        }
        return l == num;
    }
}

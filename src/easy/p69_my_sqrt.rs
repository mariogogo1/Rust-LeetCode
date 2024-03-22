/**
69. x 的平方根

给你一个非负整数 x ，计算并返回 x 的 算术平方根 。

由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。

注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。

https://leetcode.cn/problems/sqrtx/description/
*/

pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x_i64 = x as i64;
        let mut lp = 0 as i64;
        let mut rp = x as i64;
        while lp < rp {
            let mp = (lp + rp) / 2;
            let v = mp * mp;
            if v > x_i64 {
                rp = mp;
            } else {
                if (mp + 1) * (mp + 1) > x_i64 {
                    return mp as i32;
                } else {
                    lp = mp + 1;
                }
            }
        }
        return lp as i32;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::my_sqrt(0), 0);
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(2), 1);
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}

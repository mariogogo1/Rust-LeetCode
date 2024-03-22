/**
50. Pow(x, n)

实现 pow(x, n) ，即计算 x 的整数 n 次幂函数（即，xn ）。

https://leetcode.cn/problems/powx-n/description/
*/

pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn quick_pow(x: f64, n: i64) -> f64 {
            if n == 0 {
                return 1.0;
            }
            let a = quick_pow(x, n / 2);
            if n % 2 == 0 {
                return a * a;
            }
            return a * a * x;
        }

        if n < 0 {
            return quick_pow(1.0 / x, -(n as i64));
        }
        return quick_pow(x, n as i64);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
        assert_eq!(Solution::my_pow(2.1, 3), 9.261);
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::my_pow(2.00000, -2147483648), 0.0000);
    }
}

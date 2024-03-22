/**
372. 超级次方

你的任务是计算 ab 对 1337 取模，a 是一个正整数，b 是一个非常大的正整数且会以数组形式给出。

https://leetcode.cn/problems/super-pow/description/
*/

pub struct Solution;

impl Solution {
    fn quick_pow(x: i64, n: i64) -> i64 {
        if n == 0 {
            return 1;
        }
        let xx = Self::quick_pow((x * x) % 1337_i64, n / 2);
        if n % 2 == 1 {
            return (xx * x) % 1337_i64;
        }
        return (xx) % 1337_i64;
    }

    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        // TODO: |         |
        //       ▽        ▽
        // [(a^b0)^10*(a^b1)]^10.....
        let a_i64 = a as i64;
        let mut ans = 1 as i64;
        for &i in b.iter() {
            ans = Self::quick_pow(ans, 10);
            ans *= Self::quick_pow(a_i64, i as i64);
        }
        return (ans % 1337_i64) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::super_pow(2, vec![3]), 8);
        assert_eq!(Solution::super_pow(2, vec![1, 0]), 1024);
        assert_eq!(Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]), 1);
        assert_eq!(Solution::super_pow(2147483647, vec![2, 0, 0]), 1198);
    }
    #[test]
    fn test_case() {}
}

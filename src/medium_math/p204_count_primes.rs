/**
204. 计数质数

给定整数 n ，返回 所有小于非负整数 n 的质数的数量 。

https://leetcode.cn/problems/count-primes/description/
*/
/// 因為範圍給定是10^8為上限，所以很多檢查可以不用作

pub struct Solution;
/// 埃式篩
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut primes_bool: Vec<bool> = vec![false; n + 1];
        let mut ans = n - 2;
        if n <= 2 {
            return 0;
        }
        let mut i: usize = 2;
        while i * i <= n {
            if primes_bool[i] {
                i += 1;
                continue;
            }

            // 如果要更快 就要改寫這一段 可以把 - 1直接改成 +n/i ，但要調整
            let mut s = i * 2;
            while s < n {
                if !primes_bool[s] {
                    primes_bool[s] = true;
                    ans -= 1;
                }
                s += i;
            }

            i += 1;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::count_primes(10), 4);
        assert_eq!(Solution::count_primes(2), 0);
    }
    #[test]
    fn test_case() {}
}

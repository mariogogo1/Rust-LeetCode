/**
264. 丑数 II

给你一个整数 n ，请你找出并返回第 n 个 丑数 。

丑数 就是质因子只包含 2、3 和 5 的正整数。

https://leetcode.cn/problems/ugly-number-ii/description/
*/

pub struct Solution;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ptrs: Vec<usize> = vec![0, 0, 0];
        let mut ugly_vec = vec![1];
        let mut ans = 1;
        let mut i = 1;
        while i < n {
            let a = ugly_vec[ptrs[0]] * 2;
            let b = ugly_vec[ptrs[1]] * 3;
            let c = ugly_vec[ptrs[2]] * 5;

            if a <= b && a <= c {
                ptrs[0] += 1;
                ans = a;
            }
            if b <= a && b <= c {
                ptrs[1] += 1;
                ans = b;
            }
            if c <= a && c <= b {
                ptrs[2] += 1;
                ans = c;
            }
            ugly_vec.push(ans);
            i += 1;
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
        assert_eq!(Solution::nth_ugly_number(1), 1);
    }
    #[test]
    fn test_case() {}
}

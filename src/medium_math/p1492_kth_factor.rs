/**
1492. n 的第 k 个因子

给你两个正整数 n 和 k 。

如果正整数 i 满足 n % i == 0 ，那么我们就说正整数 i 是整数 n 的因子。

考虑整数 n 的所有因子，将它们 升序排列 。请你返回第 k 个因子。如果 n 的因子数少于 k ，请你返回 -1 。


https://leetcode.cn/problems/the-kth-factor-of-n/description/
*/
pub struct Solution;

use f64;

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let n_usize = n as usize;
        let mut k_usize = k as usize;
        let n_sqrt = (n as f64 + 0.05).sqrt() as usize + 1;
        let mut back_stack: Vec<usize> = vec![];
        for i in 1..n_sqrt {
            if n_usize % i == 0 {
                k_usize -= 1;
                if k_usize == 0 {
                    return i as i32;
                }
                back_stack.push(n_usize / i);
            }
        }
        if n_usize == (n_sqrt - 1) * (n_sqrt - 1) {
            k_usize += 1;
        }
        if k_usize > back_stack.len() {
            return -1;
        }
        return back_stack[back_stack.len() - k_usize] as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::kth_factor(12, 3), 3);
        assert_eq!(Solution::kth_factor(7, 2), 7);
        assert_eq!(Solution::kth_factor(4, 4), -1);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::kth_factor(2, 2), 2);
        assert_eq!(Solution::kth_factor(1, 1), 1);
        assert_eq!(Solution::kth_factor(1, 2), -1);
    }
}

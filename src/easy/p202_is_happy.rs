/**
202. 快乐数
编写一个算法来判断一个数 n 是不是快乐数。

「快乐数」 定义为：

对于一个正整数，每一次将该数替换为它每个位置上的数字的平方和。
然后重复这个过程直到这个数变为 1，也可能是 无限循环 但始终变不到 1。
如果这个过程 结果为 1，那么这个数就是快乐数。
如果 n 是 快乐数 就返回 true ；不是，则返回 false 。
https://leetcode.cn/problems/happy-number/description/
*/

pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut hashset: HashSet<i32> = HashSet::new();
        loop {
            if hashset.contains(&n) {
                return n == 1;
            } else {
                hashset.insert(n);
            }
            n = Self::calculate(n);
        }
        unreachable!();
    }

    fn calculate(mut n: i32) -> i32 {
        let mut ans = 0;
        while n != 0 {
            ans += (n % 10) * (n % 10);
            n /= 10;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::is_happy(19), true);
    }
    #[test]
    fn test_case() {}
}

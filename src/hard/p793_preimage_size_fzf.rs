/**
793. 阶乘函数后 K 个零

 f(x) 是 x! 末尾是 0 的数量。回想一下 x! = 1 * 2 * 3 * ... * x，且 0! = 1 。

例如， f(3) = 0 ，因为 3! = 6 的末尾没有 0 ；而 f(11) = 2 ，因为 11!= 39916800 末端有 2 个 0 。
给定 k，找出返回能满足 f(x) = k 的非负整数 x 的数量。

提示：

0 <= k <= 10^9

https://leetcode.cn/problems/preimage-size-of-factorial-zeroes-function/description/
*/
pub struct Solution;

/// n!有多少個0，可以用除以5的數量累加起來可以知道有多少個0
/// 舉例： 30! 一共有 30/5 =6**  6/5=1**   一共有6+1=7個0為尾數        
/// 前導題目：522

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        let k_i64 = k as i64;
        let mut l: i64 = 0;
        let mut r = i64::MAX;

        while l < r {
            let mut m = (l + r) / 2;
            let s = Self::find_zero(m);
            println!("{}", s);
            if s == k_i64 {
                return 5;
            } else if s > k_i64 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        return 0;
    }

    fn find_zero(mut x: i64) -> i64 {
        let mut ans = 0;
        while x > 0 {
            x /= 5;
            ans += x;
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::preimage_size_fzf(0), 5);
        assert_eq!(Solution::preimage_size_fzf(5), 0);
        assert_eq!(Solution::preimage_size_fzf(3), 5);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::preimage_size_fzf(1000000000), 5);
    }
}

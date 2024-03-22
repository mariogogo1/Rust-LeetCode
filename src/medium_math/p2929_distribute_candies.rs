/**
2929. 给小朋友们分糖果 II

给你两个正整数 n 和 limit 。

请你将 n 颗糖果分给 3 位小朋友，确保没有任何小朋友得到超过 limit 颗糖果，请你返回满足此条件下的 总方案数 。

https://leetcode.cn/problems/distribute-candies-among-children-ii/description/
*/

pub struct Solution;

impl Solution {
    fn combination(n: i64) -> i64 {
        if n > 1 {
            return n * (n - 1) / 2;
        }
        return 0;
    }

    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        /// C(n+2,2)-3C(n-limit+1,2)+3C(n-2*limit,2)-C(n+3*limit-1,2)
        let n_i64 = n as i64;
        let limit_i64 = limit as i64;
        return Self::combination(n_i64 + 2) - 3 * Self::combination(n_i64 - limit_i64 + 1)
            + 3 * Self::combination(n_i64 - 2 * limit_i64)
            - Self::combination(n_i64 - 3 * limit_i64 - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::distribute_candies(5, 2), 3);
        assert_eq!(Solution::distribute_candies(3, 3), 10);
        assert_eq!(Solution::distribute_candies(2, 1), 3);
    }
    #[test]
    fn test_case() {}
}

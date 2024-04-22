/**
1735. 生成乘积数组的方案数

给你一个二维整数数组 queries ，其中 queries[i] = [ni, ki] 。第 i 个查询 queries[i] 要求构造长度为 ni 、每个元素都是正整数的数组，且满足所有元素的乘积为 ki ，请你找出有多少种可行的方案。由于答案可能会很大，方案数需要对 109 + 7 取余 。

请你返回一个整数数组 answer，满足 answer.length == queries.length ，其中 answer[i]是第 i 个查询的结果。

https://leetcode.cn/problems/count-ways-to-make-array-with-product/description/?envType=study-plan-v2&envId=2024-spring-sprint-100
*/
pub struct Solution;
use std::collections::HashSet;
impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;
    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut max_dp_n = 0;
        let mut max_dp_k = 0;
        let mut prime_factor_count = Vec::new();

        for i in 0..queries.len() {
            max_dp_k = max_dp_k.max(queries[i][0] as usize);
            if queries[i][1] == 1 {
                prime_factor_count.push(vec![0]);
            } else {
                let mut number = queries[i][1];
                let mut count_vec = Vec::new();
                // 這個方法原比預處理 要快!
                let mut factor = 2;
                while factor * factor <= number {
                    if number % factor == 0 {
                        let mut count = 0;
                        while number % factor == 0 {
                            count += 1;
                            number /= factor;
                        }
                        if count > 0 {
                            count_vec.push(count);
                            max_dp_n = max_dp_n.max(count);
                        }
                    }
                    factor += 1;
                }
                if number > 1 {
                    // 最後一個質因數
                    count_vec.push(1);
                    max_dp_n = max_dp_n.max(1);
                }

                prime_factor_count.push(count_vec);
            }
        }

        // 用來做DP 使用只要做一次最大值得即可
        let mut ans = Vec::new();
        let mut dp: Vec<Vec<i64>> = vec![vec![0; max_dp_n + max_dp_k]; max_dp_n + 1];
        for j in 0..(max_dp_n + max_dp_k) {
            dp[0][j] = 1;
        }
        for i in 1..=max_dp_n {
            for j in i..(max_dp_n + max_dp_k) {
                dp[i][j] = dp[i - 1][j - 1] + dp[i][j - 1];
                dp[i][j] %= Self::VAL_MOD;
            }
        }

        for i in 0..prime_factor_count.len() {
            let mut one_ans = 1 as i64;
            let k = (queries[i][0] - 1) as usize;
            for j in 0..prime_factor_count[i].len() {
                one_ans *= dp[prime_factor_count[i][j]][prime_factor_count[i][j] + k];
                one_ans %= Self::VAL_MOD;
            }
            ans.push(one_ans as i32);
        }

        ans
    }
    pub fn ways_to_fill_array_slow(queries: Vec<Vec<i32>>) -> Vec<i32> {
        //找出所有質因數 質因數分解 數字上限10^4 質因數在100以下
        let mut prime_hashset = HashSet::new();
        let mut non_prime_hashset = HashSet::new();
        for i in 2..=10000 {
            if non_prime_hashset.contains(&i) {
                continue;
            } else {
                prime_hashset.insert(i);
                let mut mult = 2;
                while mult * i <= 10000 {
                    non_prime_hashset.insert(mult * i);
                    mult += 1;
                }
            }
        }

        // 用來做DP 使用只要做一次最大值得即可
        let mut max_dp_n = 0;
        let mut max_dp_k = 0;
        let mut prime_factor_count = Vec::new();

        for i in 0..queries.len() {
            max_dp_k = max_dp_k.max(queries[i][0] as usize);
            if queries[i][1] == 1 {
                prime_factor_count.push(vec![0]);
            } else {
                let mut number = queries[i][1];
                let mut count_vec = Vec::new();

                for key in &prime_hashset {
                    let mut count = 0 as usize;
                    while number % key == 0 {
                        count += 1;
                        number /= key;
                    }
                    if count > 0 {
                        count_vec.push(count);
                        max_dp_n = max_dp_n.max(count);
                    }
                }

                prime_factor_count.push(count_vec);
            }
        }

        let mut ans = Vec::new();
        let mut dp: Vec<Vec<i64>> = vec![vec![0; max_dp_n + max_dp_k]; max_dp_n + 1];
        for j in 0..(max_dp_n + max_dp_k) {
            dp[0][j] = 1;
        }
        for i in 1..=max_dp_n {
            for j in i..(max_dp_n + max_dp_k) {
                dp[i][j] = dp[i - 1][j - 1] + dp[i][j - 1];
                dp[i][j] %= Self::VAL_MOD;
            }
        }

        for i in 0..prime_factor_count.len() {
            let mut one_ans = 1 as i64;
            let k = (queries[i][0] - 1) as usize;
            for j in 0..prime_factor_count[i].len() {
                one_ans *= dp[prime_factor_count[i][j]][prime_factor_count[i][j] + k];
                one_ans %= Self::VAL_MOD;
            }
            ans.push(one_ans as i32);
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::ways_to_fill_array(vec![vec![2, 6], vec![5, 1], vec![73, 660]]),
            vec![4, 1, 50734910]
        );
    }
    #[test]
    fn test_case() {}
}

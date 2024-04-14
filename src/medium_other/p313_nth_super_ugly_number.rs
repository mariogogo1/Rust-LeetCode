/**
313. 超级丑数

超级丑数 是一个正整数，并满足其所有质因数都出现在质数数组 primes 中。

给你一个整数 n 和一个整数数组 primes ，返回第 n 个 超级丑数 。

题目数据保证第 n 个 超级丑数 在 32-bit 带符号整数范围内。

https://leetcode.cn/problems/super-ugly-number/description/
*/
pub struct Solution;
impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut ugly_nums: Vec<i32> = vec![1];
        let mut ugly_pointers = vec![0; primes.len()];
        let mut ugly_pointers_nums = vec![0; primes.len()];

        for _ in 1..n {
            let mut min_v = i32::MAX;
            for (key, value) in ugly_pointers.iter().enumerate() {
                if let Some(mul_result) = ugly_nums[*value].checked_mul(primes[key]) {
                    min_v = min_v.min(mul_result);
                    ugly_pointers_nums[key] = mul_result;
                }
            }
            ugly_nums.push(min_v);
            for (key, value) in ugly_pointers_nums.iter().enumerate() {
                if *value == min_v {
                    ugly_pointers[key] += 1;
                }
            }
        }

        *ugly_nums.last().unwrap()
    }
}

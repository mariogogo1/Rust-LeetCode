/**
1052. 爱生气的书店老板

有一个书店老板，他的书店开了 n 分钟。每分钟都有一些顾客进入这家商店。给定一个长度为 n 的整数数组 customers，

其中 customers[i] 是在第 i 分钟开始时进入商店的顾客数量，所有这些顾客在第 i 分钟结束后离开。

在某些时候，书店老板会生气。 如果书店老板在第 i 分钟生气，那么 grumpy[i] = 1，否则 grumpy[i] = 0。

当书店老板生气时，那一分钟的顾客就会不满意，若老板不生气则顾客是满意的。

书店老板知道一个秘密技巧，能抑制自己的情绪，可以让自己连续 minutes 分钟不生气，但却只能使用一次。

请你返回 这一天营业下来，最多有多少客户能够感到满意 。

Hint:
n == customers.length == grumpy.length
1 <= minutes <= n <= 2 * 104
0 <= customers[i] <= 1000
grumpy[i] == 0 or 1

https://leetcode.cn/problems/grumpy-bookstore-owner/description/
*/

pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut original_satisfied: i32 = 0;
        let (mut max_cooldown_satisfied, mut cooldown_satisfied): (i32, i32) = (0, 0);
        let m: usize = minutes as usize;
        for i in 0..m {
            original_satisfied += &customers[i] * (1 - &grumpy[i]);
            cooldown_satisfied += &customers[i] * &grumpy[i];
        }
        max_cooldown_satisfied = max(max_cooldown_satisfied, cooldown_satisfied);
        for i in m..customers.len() {
            original_satisfied += &customers[i] * (1 - &grumpy[i]);
            cooldown_satisfied += &customers[i] * &grumpy[i];
            cooldown_satisfied -= &customers[i - m] * &grumpy[i - m];
            max_cooldown_satisfied = max(max_cooldown_satisfied, cooldown_satisfied);
        }

        original_satisfied + max_cooldown_satisfied
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            ),
            16
        );
        assert_eq!(Solution::max_satisfied(vec![1], vec![0], 1), 1);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::max_satisfied(vec![3], vec![1], 1), 3);
    }
}

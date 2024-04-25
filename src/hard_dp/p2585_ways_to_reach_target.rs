/**
2585. 获得分数的方法数

考试中有 n 种类型的题目。给你一个整数 target 和一个下标从 0 开始的二维整数数组 types ，其中 types[i] = [counti, marksi] 表示第 i 种类型的题目有 counti 道，每道题目对应 marksi 分。

返回你在考试中恰好得到 target 分的方法数。由于答案可能很大，结果需要对 109 +7 取余。

注意，同类型题目无法区分。

比如说，如果有 3 道同类型题目，那么解答第 1 和第 2 道题目与解答第 1 和第 3 道题目或者第 2 和第 3 道题目是相同的。

https://leetcode.cn/problems/number-of-ways-to-earn-points/description/
*/
pub struct Solution;

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
        let n = types.len();
        let target = target as usize;
        let mut dp: Vec<i64> = vec![0; target + 1];
        dp[0] = 1;

        for i in 0..n {
            let count_i = types[i][0] as usize;
            let mark_i = types[i][1] as usize;

            for j in (1..=target).rev() {
                for k in 0..count_i {
                    if j >= (k + 1) * mark_i {
                        dp[j] += dp[j - (k + 1) * mark_i];
                    }
                }

                dp[j] %= Self::VAL_MOD;
            }
        }

        // println!("{:?}", dp);

        dp[target] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::ways_to_reach_target(6, vec![vec![6, 1], vec![3, 2], vec![2, 3]]),
            7
        );
        assert_eq!(
            Solution::ways_to_reach_target(5, vec![vec![50, 1], vec![50, 2], vec![50, 5]]),
            4
        );
    }
    #[test]
    fn test_case() {
        assert_eq!(
            Solution::ways_to_reach_target(18, vec![vec![6, 1], vec![3, 2], vec![2, 3]]),
            1
        );
    }
}

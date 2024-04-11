/**
474. 一和零

给你一个二进制字符串数组 strs 和两个整数 m 和 n 。

请你找出并返回 strs 的最大子集的长度，该子集中 最多 有 m 个 0 和 n 个 1 。

如果 x 的所有元素也是 y 的元素，集合 x 是集合 y 的 子集 。

https://leetcode.cn/problems/ones-and-zeroes/description/
*/
pub struct Solution;
/// 這類DP問題記得反過來運算。
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut m = m as usize;
        let mut n = n as usize;
        let mut dp: Vec<Vec<i32>> = vec![vec![i32::MIN; m + 1]; n + 1];
        dp[0][0] = 0; // 起始數量，
        for s in strs {
            let (zero_count, one_count) = Solution::count_zero_one(&s);
            for i in (0..=n).rev() {
                for j in (0..=m).rev() {
                    if i >= one_count && j >= zero_count && dp[i - one_count][j - zero_count] >= 0 {
                        dp[i][j] = dp[i][j].max(dp[i - one_count][j - zero_count] + 1);
                    }
                }
            }
        }
        let mut ans = 0;
        for i in (0..=n).rev() {
            for j in (0..=m).rev() {
                ans = ans.max(dp[i][j]);
            }
        }
        ans
    }

    fn count_zero_one(s: &String) -> (usize, usize) {
        let zero_count = s.chars().filter(|&c| c == '0').count();
        let one_count = s.chars().filter(|&c| c == '1').count();
        (zero_count, one_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::find_max_form(
                vec![
                    "10".to_string(),
                    "0001".to_string(),
                    "111001".to_string(),
                    "1".to_string(),
                    "0".to_string()
                ],
                5,
                3
            ),
            4
        );
    }
    #[test]
    fn test_case() {}
}

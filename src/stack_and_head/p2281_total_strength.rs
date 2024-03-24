/**
2281. 巫师的总力量和

作为国王的统治者，你有一支巫师军队听你指挥。

给你一个下标从 0 开始的整数数组 strength ，其中 strength[i] 表示第 i 位巫师的力量值。对于连续的一组巫师（也就是这些巫师的力量值是 strength 的 子数组），总力量 定义为以下两个值的 乘积 ：

巫师中 最弱 的能力值。
组中所有巫师的个人力量值 之和 。
请你返回 所有 巫师组的 总 力量之和。由于答案可能很大，请将答案对 109 + 7 取余 后返回。

子数组 是一个数组里 非空 连续子序列。

提示：

1 <= strength.length <= 10^5
1 <= strength[i] <= 10^9

https://leetcode.cn/problems/sum-of-total-strength-of-wizards/description/
*/

///  單調棧 類似1856
///  本題關鍵是 前綴和的前綴和 想法!!

pub struct Solution;

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn total_strength(strength: Vec<i32>) -> i32 {
        let n = strength.len();
        let mut ans: i64 = 0;
        let mut left_index: Vec<usize> = vec![0; n];
        let mut right_index: Vec<usize> = vec![n - 1; n];
        let mut prefix_sum: Vec<i64> = vec![0; n + 1];
        let mut prefix_sum_sum: Vec<i64> = vec![0; n + 2];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..n {
            prefix_sum[i + 1] = strength[i] as i64 + prefix_sum[i];
            // **計算以下前綴和的前綴和相減後可能出現負數**
            prefix_sum_sum[i + 2] =
                (prefix_sum[i + 1] as i64 + prefix_sum_sum[i + 1]) % Self::VAL_MOD;

            while let Some(&top) = stack.last() {
                if strength[top] >= strength[i] {
                    right_index[top] = i - 1;
                    stack.pop();
                } else {
                    left_index[i] = top + 1;
                    break;
                }
            }
            stack.push(i);
        }
        for i in 0..n {
            let x: i64 = ((prefix_sum_sum[right_index[i] + 2] - prefix_sum_sum[i + 1])
                * (i - left_index[i] + 1) as i64
                - (prefix_sum_sum[i + 1] - prefix_sum_sum[left_index[i]])
                    * (right_index[i] - i + 1) as i64)
                % Self::VAL_MOD;

            ans = (ans + x * strength[i] as i64) % Self::VAL_MOD;
        }
        ans = (ans + Self::VAL_MOD) % Self::VAL_MOD; // 避免負數
        return ans as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::total_strength(vec![1, 3, 1, 2]), 44);
        assert_eq!(Solution::total_strength(vec![5, 4, 6]), 213);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::total_strength(vec![1, 1, 1, 1]), 20);
        assert_eq!(
            Solution::total_strength(vec![
                747, 812, 112, 1230, 1426, 1477, 1388, 976, 849, 1431, 1885, 1845, 1070, 1980, 280,
                1075, 232, 1330, 1868, 1696, 1361, 1822, 524, 1899, 1904, 538, 731, 985, 279, 1608,
                1558, 930, 1232, 1497, 875, 1850, 1173, 805, 1720, 33, 233, 330, 1429, 1688, 281,
                362, 1963, 927, 1688, 256, 1594, 1823, 743, 553, 1633, 1898, 1101, 1278, 717, 522,
                1926, 1451, 119, 1283, 1016, 194, 780, 1436, 1233, 710, 1608, 523, 874, 1779, 1822,
                134, 1984
            ]),
            471441678
        );
    }
}

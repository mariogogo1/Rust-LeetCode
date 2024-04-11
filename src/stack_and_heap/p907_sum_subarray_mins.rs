/**
907. 子数组的最小值之和

给定一个整数数组 arr，找到 min(b) 的总和，其中 b 的范围为 arr 的每个（连续）子数组。

由于答案可能很大，因此 返回答案模 10^9 + 7 。
https://leetcode.cn/problems/sum-of-subarray-minimums/description/
*/
/// 類似 第84題
/// 注意：同一個子數組裡面有兩個一樣小的值
/// [1 2 4 5 6 2 7 8 9 1]
///  c a       b       d
/// 第一個2(a位置)，所有子數組數量只計算(c~b)之間
/// 第二個2(b位置)，所有子數組數量可計算(c~d)之間,
/// 這樣兩個2為最小值的子數組就不會重疊

pub struct Solution;

impl Solution {
    pub const VAL_MOD: i64 = 1_000_000_007;
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut ans: i64 = 0;
        let mut left_index: Vec<usize> = vec![0; n];
        let mut right_index: Vec<usize> = vec![n - 1; n];
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..n {
            while let Some(&top) = stack.last() {
                if arr[top] >= arr[i] {
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
            let ln = (i - left_index[i] + 1) as i64;
            let rn = (right_index[i] - i + 1) as i64;
            ans += (ln * rn) * arr[i] as i64;
            ans %= Self::VAL_MOD;
        }
        return ans as i32;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
        assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }
    #[test]
    fn test_case() {}
}

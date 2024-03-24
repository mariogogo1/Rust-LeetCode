/**
769. 最多能完成排序的块

给定一个长度为 n 的整数数组 arr ，它表示在 [0, n - 1] 范围内的整数的排列。

我们将 arr 分割成若干 块 (即分区)，并对每个块单独排序。将它们连接起来后，使得连接的结果和按升序排序后的原数组相同。

返回数组能分成的最多块数量。

Hint:

https://leetcode.cn/problems/max-chunks-to-make-sorted/description/
*/
pub struct Solution;
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut furtherest_range = -1;
        for i in 0..arr.len() {
            if furtherest_range < arr[i] {
                furtherest_range = arr[i];
            }
            if furtherest_range == i as i32 {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
        assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    }
    #[test]
    fn test_case() {}
}

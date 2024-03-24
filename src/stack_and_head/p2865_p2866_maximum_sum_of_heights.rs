/**
2865. 美丽塔 I
2866. 美丽塔 II

给你一个长度为 n 下标从 0 开始的整数数组 maxHeights 。

你的任务是在坐标轴上建 n 座塔。第 i 座塔的下标为 i ，高度为 heights[i] 。

如果以下条件满足，我们称这些塔是 美丽 的：

1 <= heights[i] <= maxHeights[i]
heights 是一个 山脉 数组。
如果存在下标 i 满足以下条件，那么我们称数组 heights 是一个 山脉 数组：

对于所有 0 < j <= i ，都有 heights[j - 1] <= heights[j]
对于所有 i <= k < n - 1 ，都有 heights[k + 1] <= heights[k]
请你返回满足 美丽塔 要求的方案中，高度和的最大值 。

https://leetcode.cn/problems/beautiful-towers-i/description/
https://leetcode.cn/problems/beautiful-towers-ii/description/
*/

pub struct Solution;
impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let mut left_area_dp: Vec<i64> = vec![0; n]; // 紀錄當下標為山頂的面積，從左邊開始爬坡
        let mut right_area_dp: Vec<i64> = vec![0; n]; // 紀錄當下標為山頂的面積，從右邊開始爬坡
        let mut stack: Vec<usize> = Vec::new(); // 遞增單調棧
        let mut ans: i64 = 0;
        // 從左邊開始爬坡
        for i in 0..n {
            while let Some(&top) = stack.last() {
                if max_heights[top] >= max_heights[i] {
                    stack.pop();
                } else {
                    left_area_dp[i] = (i - top) as i64 * max_heights[i] as i64 + left_area_dp[top];
                    break;
                }
            }
            if stack.is_empty() {
                left_area_dp[i] = (i + 1) as i64 * max_heights[i] as i64;
            }
            stack.push(i);
        }
        // 從右邊開始爬坡
        stack.clear();
        for i in (0..n).rev() {
            while let Some(&top) = stack.last() {
                if max_heights[top] >= max_heights[i] {
                    stack.pop();
                } else {
                    right_area_dp[i] =
                        (top - i) as i64 * max_heights[i] as i64 + right_area_dp[top];
                    ans = ans.max(left_area_dp[i] + right_area_dp[i] - max_heights[i] as i64);

                    break;
                }
            }
            if stack.is_empty() {
                right_area_dp[i] = (n - i) as i64 * max_heights[i] as i64;
                ans = ans.max(left_area_dp[i] + right_area_dp[i] - max_heights[i] as i64);
            }
            stack.push(i);
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::maximum_sum_of_heights(vec![5, 3, 4, 1, 1]), 13);
        assert_eq!(Solution::maximum_sum_of_heights(vec![6, 5, 3, 9, 2, 7]), 22);
        assert_eq!(Solution::maximum_sum_of_heights(vec![3, 2, 5, 5, 2, 3]), 18);
    }
    #[test]
    fn test_case() {}
}

/**
84. 柱状图中最大的矩形

给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。

求在该柱状图中，能够勾勒出来的矩形的最大面积。

1 <= heights.length <=10^5
0 <= heights[i] <= 10^4
https://leetcode.cn/problems/largest-rectangle-in-histogram/description/
*/
/// 找到以每一根柱子為長 以及左右邊界=>相減得到寬
/// 可以計算以每一根柱子為支柱的最大面積
/// 再用MAX 比較每個面積大小

/// 說明一：
/// 多根柱子長度的時候相同的時候
/// 左邊界的範圍並不需要都是最左邊的INDEX，
/// 因為面積只取最大的，只要同高柱子群的第一根，就可以計算出整個面積以下舉例
/// heights     3 6 6 6 6 6 6
/// index       0 1 2 3 4 5 6
/// stack       3 6 6 6 6 6 6  <== 連續六根高度6的柱，面積36
/// left_index  0 1 2 3 4 5 6
/// right_index 0 6 6 6 6 6 6
///               |
///               計算這一根的面積就是 連續六根高度6的柱，面積36

pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut left_index = vec![0; n];
        let mut right_index = vec![n - 1; n];
        let mut stack: Vec<usize> = Vec::new(); // 遞增單調棧，紀錄元素下標
        let mut ans = 0;

        for i in 0..n {
            while let Some(&top) = stack.last() {
                if heights[top] > heights[i] {
                    right_index[top] = i - 1; // 计算每个柱子的右边界
                    stack.pop();
                } else {
                    break;
                }
            }
            if !stack.is_empty() {
                left_index[i] = stack.last().unwrap() + 1
                //簡化代碼 计算每个柱子的左边界 **說明一
            };
            stack.push(i);
        }

        // 计算最大矩形面积
        for i in 0..n {
            let area = (right_index[i] - left_index[i] + 1) as i32 * heights[i];
            ans = ans.max(area);
        }

        ans
    }
    pub fn largest_rectangle_area_ad(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut left_index = vec![0; n];
        let mut right_index = vec![n - 1; n];
        let mut stack: Vec<usize> = Vec::new(); // 遞增單調棧，紀錄元素下標
        let mut ans = 0;

        // 计算每个柱子的左边界
        for i in 0..n {
            while let Some(&top) = stack.last() {
                if heights[top] > heights[i] {
                    right_index[top] = i - 1;
                    stack.pop();
                } else if heights[top] == heights[i] {
                    left_index[i] = left_index[top];
                    break;
                } else {
                    left_index[i] = top + 1;
                    break;
                }
            }
            stack.push(i);
        }

        // 计算最大矩形面积
        for i in 0..n {
            let area = (right_index[i] - left_index[i] + 1) as i32 * heights[i];
            ans = ans.max(area);
        }

        ans
    }
    pub fn largest_rectangle_area_basic(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut left_index = vec![0; n];
        let mut right_index = vec![n - 1; n];
        let mut stack: Vec<usize> = Vec::new();
        let mut ans = 0;

        // 计算每个柱子的左边界
        for i in 0..n {
            while let Some(&top) = stack.last() {
                if heights[top] >= heights[i] {
                    stack.pop();
                } else {
                    left_index[i] = top + 1;
                    break;
                }
            }
            stack.push(i);
        }

        // 清空栈，计算每个柱子的右边界
        stack.clear();
        for i in (0..n).rev() {
            while let Some(&top) = stack.last() {
                if heights[top] >= heights[i] {
                    stack.pop();
                } else {
                    right_index[i] = top - 1;
                    break;
                }
            }
            stack.push(i);
        }

        // 计算最大矩形面积
        for i in 0..n {
            let area = (right_index[i] - left_index[i] + 1) as i32 * heights[i];
            ans = ans.max(area);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        //assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        //assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::largest_rectangle_area(vec![7, 7, 7, 7, 7, 6]), 36);
    }
}

/**
85. 最大矩形

给定一个仅包含 0 和 1 、大小为 rows x cols 的二维二进制矩阵，找出只包含 1 的最大矩形，并返回其面积。

rows == matrix.length
cols == matrix[0].length
1 <= row, cols <= 200
matrix[i][j] 为 '0' 或 '1'
https://leetcode.cn/problems/maximal-rectangle/description/
*/
/// 計算每一列的直向柱子有多高，以下第四橫列為例子 = 第84題的 [ 0 2 0 1 4 3 ] 高度
///  
///  1 0 0 0 1 0
///  1 0 0 0 1 1
///  0 1 0 0 1 1
///  0 1 0 1 1 1
///  
/// =0 2 0 1 4 3
///  
/// 參考84-->計算每一列的最大柱狀面積

pub struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut heights = vec![0; m];
        let mut ans = 0;

        for i in 0..n {
            for j in 0..m {
                heights[j] = if matrix[i][j] == '1' {
                    heights[j] + 1
                } else {
                    0
                };
            }
            ans = ans.max(Self::largest_rectangle_area(&heights));
        }
        return ans;
    }

    pub fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
    }
    #[test]
    fn test_case() {}
}

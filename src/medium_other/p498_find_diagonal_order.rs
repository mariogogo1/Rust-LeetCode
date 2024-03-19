/**
498. 对角线遍历

给你一个大小为 m x n 的矩阵 mat ，请以对角线遍历的顺序，用一个数组返回这个矩阵中的所有元素。

Hint:
m == mat.length
n == mat[i].length
1 <= m, n <= 104
1 <= m * n <= 104
-105 <= mat[i][j] <= 105

https://leetcode.cn/problems/diagonal-traverse/description/
*/
pub struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let n = mat.len() as i32;
        let m = mat[0].len() as i32;
        let mut ans: Vec<i32> = Vec::new();
        let (mut x, mut y): (i32, i32) = (0, 0);
        // [向下，向右，向左下，向右上]
        let arr: [(i32, i32); 4] = [(1, 0), (0, 1), (1, -1), (-1, 1)];
        let mut d: usize = 3;
        loop {
            ans.push(mat[x as usize][y as usize]);
            if x == n - 1 && y == m - 1 {
                break;
            }
            match d {
                0 => {
                    if y == 0 {
                        if m == 1 {
                            d = 0
                        } else {
                            d = 3
                        }
                    } else {
                        d = 2
                    };
                }
                1 => {
                    if x == 0 {
                        if n == 1 {
                            d = 1
                        } else {
                            d = 2
                        }
                    } else {
                        d = 3
                    };
                }
                2 => {
                    if x == n - 1 {
                        d = 1
                    } else if y == 0 {
                        d = 0
                    };
                }
                3 => {
                    if y == m - 1 {
                        d = 0
                    } else if x == 0 {
                        d = 1
                    };
                }
                _ => panic!(),
            };
            x += arr[d].0;
            y += arr[d].1;
        }

        ans
    }

    // 製作新的2維陣列，新2維陣列的偶數行反向遍歷
    pub fn find_diagonal_order_zig(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let n = mat.len();
        let m = mat[0].len();
        let mut new_mat: Vec<Vec<i32>> = vec![Vec::new(); n + m - 1];
        let mut ans: Vec<i32> = Vec::new();
        for (i, &ref x_vec) in mat.iter().enumerate() {
            for (j, &y) in x_vec.iter().enumerate() {
                new_mat[i + j].push(y);
            }
        }
        for (i, &ref x_vec) in new_mat.iter().enumerate() {
            let length = x_vec.len();
            for j in 0..length {
                if i % 2 == 1 {
                    ans.push(x_vec[j]);
                } else {
                    ans.push(x_vec[length - j - 1]);
                }
            }
        }
        ans
    }

    pub fn find_diagonal_order_not_good(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let n = mat.len();
        let m = mat[0].len();
        let mut ans: Vec<i32> = Vec::new();
        // -1 左下到右上方向 ; 1 右上到左下方向
        let mut dir: i32 = -1;
        for i in 0..n + m - 1 {
            if dir == 1 {
                for j in 0..i + 1 {
                    if j < n && i - j < m {
                        ans.push(mat[j][i - j]);
                    }
                }
            } else {
                for j in 0..i + 1 {
                    if i - j < n && j < m {
                        ans.push(mat[i - j][j]);
                    }
                }
            }
            dir *= -1;
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
            Solution::find_diagonal_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 5, 9, 6, 3, 4, 7, 10, 11, 8, 12]
        );
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2], vec![3, 4]]),
            vec![1, 2, 3, 4]
        );
    }
    #[test]
    fn test_case() {
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![6, 9, 7]]),
            vec![6, 9, 7]
        );
        assert_eq!(Solution::find_diagonal_order(vec![vec![6]]), vec![6]);
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![6], vec![7], vec![9]]),
            vec![6, 7, 9]
        );
    }
}

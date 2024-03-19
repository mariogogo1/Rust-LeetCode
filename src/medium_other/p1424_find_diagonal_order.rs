/**
1424. 对角线遍历 II

给你一个列表 nums ，里面每一个元素都是一个整数列表。请你依照下面各图的规则，按顺序返回 nums 中对角线上的整数。

Hint:
1 <= nums.length <= 10^5
1 <= nums[i].length <= 10^5
1 <= nums[i][j] <= 10^9
nums 中最多有 10^5 个数字。

https://leetcode.cn/problems/diagonal-traverse-ii/description/
*/
pub struct Solution;

use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    // 內置排序 速度跟basic 方法差不多 但是資源使用較少
    ///https://doc.rust-lang.org/std/cmp/enum.Ordering.html
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let n = mat.len();
        let mut m;
        let mut new_mat: Vec<(usize, usize)> = Vec::new();
        for i in 0..n {
            m = mat[i].len();
            for j in 0..m {
                new_mat.push((i, j));
            }
        }
        // 小 -> 大
        new_mat.sort_by(|x, y| {
            if x.0 + x.1 == y.0 + y.1 {
                (x.1).cmp(&(y.1))
            } else {
                (x.0 + x.1).cmp(&(y.0 + y.1))
            }
        });

        let mut ans: Vec<i32> = Vec::new();
        for (_, x) in new_mat.iter().enumerate() {
            ans.push(mat[x.0][x.1]);
        }
        ans
    }
    // 製作新的2維陣列，
    pub fn find_diagonal_order_basic(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let n = mat.len();
        let mut m = 0;
        for (_i, &ref x_vec) in mat.iter().enumerate() {
            m = max(m, x_vec.len());
        }
        let mut new_mat: Vec<Vec<i32>> = vec![Vec::new(); n + m - 1];
        let mut ans: Vec<i32> = Vec::new();
        for (i, &ref x_vec) in mat.iter().enumerate() {
            for (j, &y) in x_vec.iter().enumerate() {
                new_mat[i + j].push(y);
            }
        }
        for (_i, &ref x_vec) in new_mat.iter().enumerate() {
            let length = x_vec.len();
            for j in 0..length {
                ans.push(x_vec[length - j - 1]);
            }
        }
        ans
    }
    // hash 表，寫得不好
    pub fn find_diagonal_order_hashmap(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut index_hashmap: HashMap<usize, Vec<i32>> = HashMap::new();
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if index_hashmap.get_mut(&(i + j)) == None {
                    index_hashmap.insert(i + j, vec![]);
                }
                index_hashmap.get_mut(&(i + j)).unwrap().push(mat[i][j])
            }
        }
        let mut ans: Vec<i32> = Vec::new();
        for i in 0..index_hashmap.len() {
            let length = index_hashmap.get(&i).unwrap().len();
            for j in 0..length {
                ans.push(index_hashmap.get(&i).unwrap()[length - 1 - j]);
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
        assert_eq!(
            Solution::find_diagonal_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 5, 2, 9, 6, 3, 10, 7, 4, 11, 12]
        );
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2], vec![3, 4]]),
            vec![1, 3, 2, 4]
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
            Solution::find_diagonal_order(vec![vec![6, 4, 5], vec![7], vec![9, 16, 18, 19]]),
            vec![6, 7, 4, 9, 5, 16, 18, 19]
        );
    }
}

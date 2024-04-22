/**
96. 不同的二叉搜索树

给你一个整数 n ，求恰由 n 个节点组成且节点值从 1 到 n 互不相同的 二叉搜索树 有多少种？返回满足题意的二叉搜索树的种数。

https://leetcode.cn/problems/unique-binary-search-trees/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;

        for i in 1..=n as usize {
            for j in 0..i {
                dp[i] += dp[j] * dp[i - 1 - j];
            }
        }

        dp[n as usize]
    }
}

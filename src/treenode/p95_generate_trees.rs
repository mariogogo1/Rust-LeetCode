/**
95. 不同的二叉搜索树 II

给你一个整数 n ，请你生成并返回所有由 n 个节点组成且节点值从 1 到 n 互不相同的不同 二叉搜索树 。可以按 任意顺序 返回答案。

https://leetcode.cn/problems/unique-binary-search-trees-ii/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn build_trees(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start >= end {
            // KEY!!
            return vec![None];
        }

        let mut ans = Vec::new();

        for i in start..=end {
            let left_trees = Self::build_trees(start, i - 1);
            let right_trees = Self::build_trees(i + 1, end);
            for left in &left_trees {
                for right in &right_trees {
                    let s = Some(Rc::new(RefCell::new(TreeNode {
                        val: i,
                        // 使用CLONE() 可能有更好的寫法
                        left: left.clone(),
                        right: right.clone(),
                    })));
                    ans.push(s);
                }
            }
        }

        ans
    }

    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::build_trees(1, n)
    }
}

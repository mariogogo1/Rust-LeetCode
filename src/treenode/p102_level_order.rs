/**
102. 二叉树的层序遍历

给你二叉树的根节点 root ，返回其节点值的 层序遍历 。 （即逐层地，从左到右访问所有节点）。

https://leetcode.cn/problems/binary-tree-level-order-traversal/description/

*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut ans = Vec::new();
        let mut start = vec![root.unwrap()];

        while !start.is_empty() {
            let mut new_start = Vec::new();
            let mut level = Vec::new();

            for i in 0..start.len() {
                level.push(start[i].borrow().val);

                let mut s = start[i].borrow_mut();

                if let Some(left) = s.left.take() {
                    new_start.push(left);
                }

                if let Some(right) = s.right.take() {
                    new_start.push(right);
                }
            }
            ans.push(level);
            start = new_start;
        }

        ans
    }
}

/**
144. 二叉树的前序遍历

给你二叉树的根节点 root ，返回它节点值的 前序 遍历。

https://leetcode.cn/problems/binary-tree-preorder-traversal/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if let Some(node) = root {
            ans.push(node.borrow().val);
            Self::dfs(node.borrow_mut().left.take(), ans);
            Self::dfs(node.borrow_mut().right.take(), ans);
        }
    }

    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();

        Self::dfs(root, &mut ans);

        ans
    }
}

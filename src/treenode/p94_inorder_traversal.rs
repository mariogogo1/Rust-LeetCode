/**
94. 二叉树的中序遍历

给定一个二叉树的根节点 root ，返回 它的 中序 遍历 。

https://leetcode.cn/problems/binary-tree-inorder-traversal/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::dfs(node.borrow_mut().left.take(), ans);

            ans.push(node.borrow().val);
            Self::dfs(node.borrow_mut().right.take(), ans);
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();

        Self::dfs(root, &mut ans);

        ans
    }
}

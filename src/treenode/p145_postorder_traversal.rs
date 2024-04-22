/**
145. 二叉树的后序遍历

给你一棵二叉树的根节点 root ，返回其节点值的 后序遍历 。

https://leetcode.cn/problems/binary-tree-postorder-traversal/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::dfs(node.borrow_mut().left.take(), ans);
            Self::dfs(node.borrow_mut().right.take(), ans);
            ans.push(node.borrow().val);
        }
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();

        Self::dfs(root, &mut ans);

        ans
    }
}

/**
226. 翻转二叉树

给你一棵二叉树的根节点 root ，翻转这棵二叉树，并返回其根节点。

https://leetcode.cn/problems/invert-binary-tree/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref node) = root {
            let left = Self::invert_tree(node.borrow_mut().left.take());
            let right = Self::invert_tree(node.borrow_mut().right.take());
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
        }
        root
    }

    pub fn invert_tree_b(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 所有權轉移
        if let Some(node) = root.take() {
            let left = Self::invert_tree(node.borrow_mut().left.take());
            let right = Self::invert_tree(node.borrow_mut().right.take());

            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
            return Some(node);
        }
        root
    }
}

/**
101. 对称二叉树

给你一个二叉树的根节点 root ， 检查它是否轴对称。

https://leetcode.cn/problems/symmetric-tree/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            return Self::is_symmetric_trees(left, right);
        }
        true
    }

    fn is_symmetric_trees(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(left_node) = left {
            if let Some(right_node) = right {
                if left_node.borrow().val == right_node.borrow().val {
                    return Self::is_symmetric_trees(
                        left_node.borrow_mut().left.take(),
                        right_node.borrow_mut().right.take(),
                    ) && Self::is_symmetric_trees(
                        right_node.borrow_mut().left.take(),
                        left_node.borrow_mut().right.take(),
                    );
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        if let Some(right_node) = right {
            return false;
        }
        return true;
    }
}

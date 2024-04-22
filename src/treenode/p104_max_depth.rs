/**
104. 二叉树的最大深度

给定一个二叉树 root ，返回其最大深度。

二叉树的 最大深度 是指从根节点到最远叶子节点的最长路径上的节点数。

https://leetcode.cn/problems/maximum-depth-of-binary-tree/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let l = Self::max_depth(node.borrow_mut().left.take());
            let r = Self::max_depth(node.borrow_mut().right.take());

            return 1 + l.max(r);
        }
        0
    }
}

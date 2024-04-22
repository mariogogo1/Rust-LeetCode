/**
112. 路径总和

给你二叉树的根节点 root 和一个表示目标和的整数 targetSum 。判断该树中是否存在 根节点到叶子节点 的路径，这条路径上所有节点值相加等于目标和 targetSum 。如果存在，返回 true ；否则，返回 false 。

叶子节点 是指没有子节点的节点。

https://leetcode.cn/problems/path-sum/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, t_f: &mut bool) {
        if *t_f {
            return;
        }
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let v = node.borrow().val;
            if left.is_none() && right.is_none() && target_sum == v {
                *t_f = true;
            } else {
                if left.is_some() {
                    Self::dfs(left, target_sum - v, t_f);
                }
                if right.is_some() {
                    Self::dfs(right, target_sum - v, t_f);
                }
            }
        }
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut t_f = false;

        Self::dfs(root, target_sum, &mut t_f);

        t_f
    }
}

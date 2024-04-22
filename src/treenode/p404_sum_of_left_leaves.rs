/**
404. 左叶子之和

给定二叉树的根节点 root ，返回所有左叶子之和。

https://leetcode.cn/problems/sum-of-left-leaves/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, is_left: bool, ans: &mut i32) {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let v = node.borrow().val;
            if left.is_none() && right.is_none() && is_left {
                *ans += v;
            } else {
                if left.is_some() {
                    Self::dfs(left, true, ans);
                }
                if right.is_some() {
                    Self::dfs(right, false, ans);
                }
            }
        }
    }
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;

        Self::dfs(root, false, &mut ans);

        ans
    }
}

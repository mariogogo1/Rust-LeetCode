/**
103. 二叉树的锯齿形层序遍历

给你二叉树的根节点 root ，返回其节点值的 锯齿形层序遍历 。（即先从左往右，再从右往左进行下一层遍历，以此类推，层与层之间交替进行）。

https://leetcode.cn/problems/binary-tree-zigzag-level-order-traversal/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut ans = Vec::new();
        let mut start = vec![root.unwrap()];
        let mut rev = false;

        while !start.is_empty() {
            let mut new_start = Vec::new();
            let mut level = Vec::new();

            for i in (0..start.len()).rev() {
                level.push(start[i].borrow().val);

                let mut s = start[i].borrow_mut();

                if rev {
                    if let Some(right) = s.right.take() {
                        new_start.push(right);
                    }
                }

                if let Some(left) = s.left.take() {
                    new_start.push(left);
                }

                if !rev {
                    if let Some(right) = s.right.take() {
                        new_start.push(right);
                    }
                }
            }
            rev ^= true;
            ans.push(level);
            start = new_start;
        }

        ans
    }
}

/**
100. 相同的树

给你两棵二叉树的根节点 p 和 q ，编写一个函数来检验这两棵树是否相同。

如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的

https://leetcode.cn/problems/same-tree/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(ref p_node) = p {
            if let Some(ref q_node) = q {
                if p_node.borrow().val == q_node.borrow().val {
                    let p_left = p_node.borrow_mut().left.take();
                    let q_left = q_node.borrow_mut().left.take();

                    return Self::is_same_tree(p_left, q_left)
                        && Self::is_same_tree(
                            p_node.borrow_mut().right.take(),
                            q_node.borrow_mut().right.take(),
                        );
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        if let Some(ref q_node) = q {
            return false;
        }
        true
    }
}

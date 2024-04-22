/**
98. 验证二叉搜索树

给你一个二叉树的根节点 root ，判断其是否是一个有效的二叉搜索树。

有效 二叉搜索树定义如下：

节点的左
子树
只包含 小于 当前节点的数。
节点的右子树只包含 大于 当前节点的数。
所有左子树和右子树自身必须也是二叉搜索树。

https://leetcode.cn/problems/validate-binary-search-tree/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (t_f, _, _) = Self::is_valid_bst_limit(root);
        t_f
    }

    fn is_valid_bst_limit(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i64, i64) {
        if let Some(node) = root {
            let (t_f_l, min_l, max_l) = Self::is_valid_bst_limit(node.borrow_mut().left.take());
            let (t_f_r, min_r, max_r) = Self::is_valid_bst_limit(node.borrow_mut().right.take());
            let v = node.borrow().val as i64;
            if v > max_l && v < min_r && t_f_l && t_f_r {
                return (true, min_l.min(v), max_r.max(v));
            }
            return (false, i64::MIN, i64::MAX);
        }
        (true, i64::MAX, i64::MIN)
    }
}

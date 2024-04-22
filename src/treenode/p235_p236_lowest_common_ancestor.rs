/**
235. 二叉搜索树的最近公共祖先
236. 二叉树的最近公共祖先

给定一个二叉树, 找到该树中两个指定节点的最近公共祖先。

百度百科中最近公共祖先的定义为：“对于有根树 T 的两个节点 p、q，最近公共祖先表示为一个节点 x，满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点也可以是它自己的祖先）。”

https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/description/
https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/description/

*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || p == root || q == root {
            return root;
        }

        let mut node = root.as_ref().unwrap();
        let left =
            Self::lowest_common_ancestor(node.borrow_mut().left.take(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(node.borrow_mut().right.take(), p, q);
        if left.is_some() && right.is_some() {
            return root;
        } else if left.is_none() && right.is_some() {
            return right;
        }
        return left;
    }
}

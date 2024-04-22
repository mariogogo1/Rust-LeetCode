/**
113. 路径总和 II

给你两棵二叉树的根节点 p 和 q ，编写一个函数来检验这两棵树是否相同。

如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的

https://leetcode.cn/problems/same-tree/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        one_ans: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let v = node.borrow().val;
            one_ans.push(v);
            if left.is_none() && right.is_none() && target_sum == v {
                ans.push(one_ans.clone());
            } else {
                if left.is_some() {
                    Self::dfs(left, target_sum - v, one_ans, ans);
                }
                if right.is_some() {
                    Self::dfs(right, target_sum - v, one_ans, ans);
                }
            }
            one_ans.pop();
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut one_ans = Vec::new();

        Self::dfs(root, target_sum, &mut one_ans, &mut ans);

        ans
    }
}

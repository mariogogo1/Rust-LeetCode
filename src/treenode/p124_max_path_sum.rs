/**
124. 二叉树中的最大路径和

二叉树中的 路径 被定义为一条节点序列，序列中每对相邻节点之间都存在一条边。同一个节点在一条路径序列中 至多出现一次 。该路径 至少包含一个 节点，且不一定经过根节点。

路径和 是路径中各节点值的总和。

给你一个二叉树的根节点 root ，返回其 最大路径和 。

https://leetcode.cn/problems/binary-tree-maximum-path-sum/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
// 其實遞歸想法不難想到，比較難想到的是要跟0比大小，如果子樹的最大和小於0，那寧可不要加總!

impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if let Some(node) = root {
            let mut x = node.borrow_mut();
            let left_sum = Self::dfs(x.left.take(), ans).max(0);
            let right_sum = Self::dfs(x.right.take(), ans).max(0);
            *ans = *ans.max(&mut (x.val + left_sum + right_sum));
            return (x.val + left_sum).max(x.val + right_sum);
        }
        0
    }

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = i32::MIN;

        _ = Self::dfs(root, &mut ans);

        ans
    }
}

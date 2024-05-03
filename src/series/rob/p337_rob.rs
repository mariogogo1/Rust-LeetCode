/**
337. 打家劫舍 III

小偷又发现了一个新的可行窃的地区。这个地区只有一个入口，我们称之为 root 。

除了 root 之外，每栋房子有且只有一个“父“房子与之相连。一番侦察之后，聪明的小偷意识到“这个地方的所有房屋的排列类似于一棵二叉树”。 如果 两个直接相连的房子在同一天晚上被打劫 ，房屋将自动报警。

给定二叉树的 root 。返回 在不触动警报的情况下 ，小偷能够盗取的最高金额 。

https://leetcode.cn/problems/house-robber-iii/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = root {
            let v = node.borrow().val;
            let (r, r_n) = Self::dfs(node.borrow_mut().right.take());
            let (l, l_n) = Self::dfs(node.borrow_mut().left.take());
            return ((r_n + l_n + v).max(r + l), r + l);
        }
        return (0, 0);
    }

    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (ans, _) = Self::dfs(root);
        return ans;
    }
}

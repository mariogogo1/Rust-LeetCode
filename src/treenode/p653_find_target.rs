/**
653. 两数之和 IV - 输入二叉搜索树

给定一个二叉搜索树 root 和一个目标结果 k，如果二叉搜索树中存在两个元素且它们的和等于给定的目标结果，则返回 true。

https://leetcode.cn/problems/two-sum-iv-input-is-a-bst/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashSet;

impl Solution {
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
        sum_hash: &mut HashSet<i32>,
        ans: &mut bool,
    ) {
        if *ans {
            return;
        }
        if let Some(node) = root {
            let v = node.borrow().val;
            if sum_hash.contains(&v) {
                *ans = true;
            } else {
                sum_hash.insert(k - v);
            }
            Self::dfs(node.borrow_mut().left.take(), k, sum_hash, ans);
            Self::dfs(node.borrow_mut().right.take(), k, sum_hash, ans);
        }
    }

    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut ans = false;

        let mut sum_hash = HashSet::new();

        Self::dfs(root, k, &mut sum_hash, &mut ans);

        ans
    }
}

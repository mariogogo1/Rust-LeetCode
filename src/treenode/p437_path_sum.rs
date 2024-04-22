/**
437. 路径总和 III

给定一个二叉树的根节点 root ，和一个整数 targetSum ，求该二叉树里节点值之和等于 targetSum 的 路径 的数目。

路径 不需要从根节点开始，也不需要在叶子节点结束，但是路径方向必须是向下的（只能从父节点到子节点）

https://leetcode.cn/problems/path-sum-iii/description/
*/
pub struct Solution;
use std::collections::HashMap;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i64,
        path_sum: i64,
        ans: &mut i64,
        sum_hashmap: &mut HashMap<i64, i64>,
    ) {
        if let Some(node) = root {
            let v = node.borrow().val as i64;

            *ans += *sum_hashmap.entry(path_sum + v - target_sum).or_insert(0);
            *sum_hashmap.entry(path_sum + v).or_insert(0) += 1;
            Self::dfs(
                node.borrow_mut().left.take(),
                target_sum,
                path_sum + v,
                ans,
                sum_hashmap,
            );
            Self::dfs(
                node.borrow_mut().right.take(),
                target_sum,
                path_sum + v,
                ans,
                sum_hashmap,
            );
            *sum_hashmap.entry(path_sum + v).or_insert(0) -= 1;
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut ans = 0;
        let mut sum_hashmap: HashMap<i64, i64> = HashMap::new();
        sum_hashmap.insert(0, 1);

        Self::dfs(root, target_sum as i64, 0, &mut ans, &mut sum_hashmap);

        ans as i32
    }
}

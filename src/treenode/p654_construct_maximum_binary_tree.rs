/**
654. 最大二叉树

给定一个不重复的整数数组 nums 。 最大二叉树 可以用下面的算法从 nums 递归地构建:

创建一个根节点，其值为 nums 中的最大值。
递归地在最大值 左边 的 子数组前缀上 构建左子树。
递归地在最大值 右边 的 子数组后缀上 构建右子树。
返回 nums 构建的 最大二叉树 。

https://leetcode.cn/problems/maximum-binary-tree/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct_maximum_binary_tree_slice(&nums[..])
    }

    fn construct_maximum_binary_tree_slice(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }
        // 找到最大值的INDEX
        let idx = nums
            .iter()
            .enumerate()
            .max_by_key(|(_, &val)| val)
            .map(|(idx, _)| idx)
            .unwrap();

        let mut root = TreeNode {
            val: nums[idx],
            left: None,
            right: None,
        };
        root.left = Self::construct_maximum_binary_tree_slice(&nums[..idx]);
        root.right = Self::construct_maximum_binary_tree_slice(&nums[(idx + 1)..]);
        Some(Rc::new(RefCell::new(root)))
    }
}

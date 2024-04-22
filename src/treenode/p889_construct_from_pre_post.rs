/**
889. 根据前序和后序遍历构造二叉树

给定两个整数数组，preorder 和 postorder ，其中 preorder 是一个具有 无重复 值的二叉树的前序遍历，postorder 是同一棵树的后序遍历，重构并返回二叉树。

如果存在多个答案，您可以返回其中 任何 一个。

https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-postorder-traversal/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct_from_pre_post_slice(&preorder[..], &postorder[..])
    }

    fn construct_from_pre_post_slice(
        preorder: &[i32],
        postorder: &[i32],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        let mut root = TreeNode::new(preorder[0]);

        if preorder.len() > 1 {
            let idx = postorder.iter().position(|&x| x == preorder[1]).unwrap();

            root.left = Self::construct_from_pre_post_slice(
                &preorder[1..(idx + 2)],
                &postorder[..(idx + 1)],
            );
            root.right = Self::construct_from_pre_post_slice(
                &preorder[(idx + 2)..],
                &postorder[(idx + 1)..postorder.len() - 1],
            );
        }

        Some(Rc::new(RefCell::new(root)))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case() {}
}

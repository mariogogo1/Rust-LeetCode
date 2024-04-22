/**
106. 从中序与后序遍历序列构造二叉树

给定两个整数数组 inorder 和 postorder ，其中 inorder 是二叉树的中序遍历， postorder 是同一棵树的后序遍历，请你构造并返回这颗 二叉树 。

https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_slice(&inorder[..], &postorder[..])
    }

    fn build_tree_slice(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }

        let root_val = postorder.last().unwrap(); // 后序遍历的最后一个元素为根节点的值
        let mut root = TreeNode::new(*root_val);

        let idx = inorder.iter().position(|&x| x == *root_val).unwrap(); // 在中序遍历中找到根节点的位置

        // 构造左子树和右子树的中序遍历和后序遍历数组
        root.left = Self::build_tree_slice(&inorder[..idx], &postorder[..idx]);
        root.right =
            Self::build_tree_slice(&inorder[(idx + 1)..], &postorder[idx..postorder.len() - 1]);

        Some(Rc::new(RefCell::new(root)))
    }
}

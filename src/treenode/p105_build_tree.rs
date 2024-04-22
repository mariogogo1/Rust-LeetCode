/**
105. 从前序与中序遍历序列构造二叉树

给定两个整数数组 preorder 和 inorder ，其中 preorder 是二叉树的先序遍历， inorder 是同一棵树的中序遍历，请构造二叉树并返回其根节点。

https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_slice(&preorder[..], &inorder[..])
    }
    // 使用切片減少內存
    fn build_tree_slice(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        let idx = inorder.iter().position(|&x| x == preorder[0]).unwrap();

        let mut s = TreeNode {
            val: preorder[0],
            left: None,
            right: None,
        };

        s.left = Self::build_tree_slice(&preorder[1..(idx + 1)], &inorder[..idx]);
        s.right = Self::build_tree_slice(&preorder[(idx + 1)..], &inorder[(idx + 1)..]);

        Some(Rc::new(RefCell::new(s)))
    }
}

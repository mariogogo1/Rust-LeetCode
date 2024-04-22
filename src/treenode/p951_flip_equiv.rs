/**
951. 翻转等价二叉树

我们可以为二叉树 T 定义一个 翻转操作 ，如下所示：选择任意节点，然后交换它的左子树和右子树。

只要经过一定次数的翻转操作后，能使 X 等于 Y，我们就称二叉树 X 翻转 等价 于二叉树 Y。

这些树由根节点 root1 和 root2 给出。如果两个二叉树是否是翻转 等价 的函数，则返回 true ，否则返回 false 。

https://leetcode.cn/problems/flip-equivalent-binary-trees/description/
*/
pub struct Solution;

use crate::utils::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (Some(node_1), Some(node_2)) => {
                if node_1.borrow().val == node_2.borrow().val {
                    let mut x_1_l = node_1.borrow_mut().left.take();
                    let mut x_2_l = node_2.borrow_mut().left.take();
                    let mut x_1_r = node_1.borrow_mut().right.take();
                    let mut x_2_r = node_2.borrow_mut().right.take();

                    let t_f_same = Self::flip_equiv(x_1_l.clone(), x_2_l.clone())
                        && Self::flip_equiv(x_1_r.clone(), x_2_r.clone());
                    let t_f_opposite = Self::flip_equiv(x_1_l.clone(), x_2_r.clone())
                        && Self::flip_equiv(x_1_r.clone(), x_2_l.clone());
                    return t_f_same || t_f_opposite;
                }
                return false;
            }
            (None, None) => return true,
            _ => return false,
        }
    }
}

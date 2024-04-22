/**
24. 两两交换链表中的节点

给你一个链表，两两交换其中相邻的节点，并返回交换后链表的头节点。你必须在不修改节点内部的值的情况下完成本题（即，只能进行节点交换）。

https://leetcode.cn/problems/swap-nodes-in-pairs/description/
*/
pub struct Solution;

use crate::utils::structs::ListNode;
impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 操作HEAD 倆倆交換
        if let Some(mut node) = head {
            if let Some(mut next_node) = node.next.take() {
                let mut next_next_node = next_node.next.take();

                node.next = Self::swap_pairs(next_next_node);
                next_node.next = Some(node);

                return Some(next_node);
            } else {
                return Some(node);
            }
        }

        head
    }
}

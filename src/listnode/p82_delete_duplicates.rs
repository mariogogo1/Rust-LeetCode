/**
82. 删除排序链表中的重复元素 II

给定一个已排序的链表的头 head ， 删除原始链表中所有重复数字的节点，只留下不同的数字 。返回 已排序的链表 。

https://leetcode.cn/problems/remove-duplicates-from-sorted-list-ii/description/
*/
pub struct Solution;

use crate::utils::structs::ListNode;
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            if let Some(mut next_node) = node.next.take() {
                if node.val == next_node.val {
                    while let Some(mut next_next_node) = next_node.next.take() {
                        if next_node.val == next_next_node.val {
                            next_node = next_next_node;
                        } else {
                            return Self::delete_duplicates(Some(next_next_node));
                        }
                    }
                    return None;
                } else {
                    node.next = Self::delete_duplicates(Some(next_node));
                    return Some(node);
                }
            } else {
                return Some(node);
            }
        }
        head
    }
}

/**
206. 反转链表

给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。

https://leetcode.cn/problems/reverse-linked-list/description/
*/
pub struct Solution;

use crate::utils::structs::ListNode;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse(
            head: Option<Box<ListNode>>,
            tail: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if let Some(mut node) = head {
                let x = node.next.take();
                node.next = tail;

                return reverse(x, Some(node));
            }
            tail
        }

        reverse(head, None)
    }
}

/**
203. 移除链表元素

给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。

https://leetcode.cn/problems/remove-linked-list-elements/description/
*/
pub struct Solution;

use crate::utils::structs::ListNode;
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut cur = dummy.as_mut(); // 拆掉SOME()
        while let Some(ref mut next_node) = cur.next {
            if next_node.val != val {
                cur = cur.next.as_mut().unwrap();
            } else {
                cur.next = next_node.next.take();
            }
        }

        dummy.next
    }
}

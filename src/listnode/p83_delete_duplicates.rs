/**
83. 删除排序链表中的重复元素

给定一个已排序的链表的头 head ， 删除所有重复的元素，使每个元素只出现一次 。返回 已排序的链表 。

https://leetcode.cn/problems/remove-duplicates-from-sorted-list/description/
*/
pub struct Solution;

use crate::utils::structs::ListNode;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head.as_mut();
        // 關鍵 take()!! 取走所有權
        while let Some(node) = cur.take() {
            if let Some(next_node) = node.next.as_mut() {
                if node.val != next_node.val {
                    cur = node.next.as_mut();
                } else {
                    node.next = next_node.next.take();
                    // 拿回所有權 !
                    cur = Some(node);
                }
            }
        }

        head
    }
}

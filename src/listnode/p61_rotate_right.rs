/**
61. 旋转链表

给你一个链表的头节点 head ，旋转链表，将链表每个节点向右移动 k 个位置。

https://leetcode.cn/problems/rotate-list/description/
*/
pub struct Solution;

use crate::utils::structs::ListNode;
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut length = 0;
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            curr = node.next.as_ref();
            length += 1;
        }
        if length == 0 {
            return head;
        }
        let mut k = k % length;

        if k == 0 {
            return head;
        }

        k = length - k - 1;
        let mut ans = Box::new(ListNode {
            val: -1,
            next: None,
        });

        {
            let mut slow = head.as_mut();

            while k > 0 {
                if let Some(node) = slow {
                    slow = node.next.as_mut().take();
                }
                k -= 1;
            }

            if let Some(node) = slow {
                ans.next = node.next.take();
                node.next = None;
            }
        }

        let mut pre = ans.next.as_mut();

        // KEY 拿走所有權
        while let Some(node) = pre.take() {
            if let Some(next_node) = node.next.as_mut() {
                pre = node.next.as_mut();
            } else {
                node.next = head.take();
            }
        }

        ans.next
    }
}

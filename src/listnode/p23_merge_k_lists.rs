/**
23. 合并 K 个升序链表

给你一个链表数组，每个链表都已经按升序排列。

请你将所有链表合并到一个升序链表中，返回合并后的链表。

https://leetcode.cn/problems/merge-k-sorted-lists/description/
*/
pub struct Solution;

use crate::utils::structs::ListNode;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let n = lists.len();
        if n == 0 {
            return None;
        } else if n == 1 {
            return lists[0].clone();
        } else if n == 2 {
            return Self::merge_two_lists(lists[0].clone(), lists[1].clone());
        }

        return Self::merge_two_lists(
            Self::merge_k_lists(lists[..(n / 2)].to_vec()),
            Self::merge_k_lists(lists[(n / 2)..].to_vec()),
        );
    }

    fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        return match (list1, list2) {
            (Some(mut list1), Some(mut list2)) => {
                if list1.val < list2.val {
                    let next = list1.next.take();
                    list1.next = Self::merge_two_lists(next, Some(list2));
                    return Some(list1);
                } else {
                    let next = list2.next.take();
                    list2.next = Self::merge_two_lists(next, Some(list1));
                    return Some(list2);
                }
            }
            (x, y) => x.or(y),
        };
    }
}

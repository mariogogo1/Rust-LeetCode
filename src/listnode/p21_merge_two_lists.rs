/**
21. 合并两个有序链表

将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。

实现 FrequencyTracker 类：


Hint:
两个链表的节点数目范围是 [0, 50]
-100 <= Node.val <= 100
l1 和 l2 均按 非递减顺序 排列

https://leetcode.cn/problems/merge-two-sorted-lists/description/
*/
pub struct Solution;

use crate::utils::structs::ListNode;

impl Solution {
    pub fn merge_two_lists(
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

// 練習寫 ListNode 的 TestCase，更好理解 Rust的ListNode。
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case_1() {
        //l1 = [1,2,4], l2 = [1,3,4]
        let mut l1 = Some(Box::new(ListNode::new(1)));
        let mut cur = l1.as_mut().unwrap(); // 可變借用，要拆包成為 BOX<T>
        cur.next = Some(Box::new(ListNode::new(3)));
        cur = cur.next.as_mut().unwrap();
        cur.next = Some(Box::new(ListNode::new(4)));

        let mut l2 = Some(Box::new(ListNode::new(1)));
        let mut cur = l2.as_mut().unwrap();
        cur.next = Some(Box::new(ListNode::new(2)));
        cur = cur.next.as_mut().unwrap();
        cur.next = Some(Box::new(ListNode::new(4)));

        // 调用 merge_two_lists 函数进行合并
        let merge = Solution::merge_two_lists(l1, l2);

        // 验证合并后的结果是否符合预期
        let mut cur = merge.as_ref();
        assert_eq!(cur.unwrap().val, 1);
        cur = cur.unwrap().next.as_ref();
        assert_eq!(cur.unwrap().val, 1);
        cur = cur.unwrap().next.as_ref();
        assert_eq!(cur.unwrap().val, 2);
        cur = cur.unwrap().next.as_ref();
        assert_eq!(cur.unwrap().val, 3);
        cur = cur.unwrap().next.as_ref();
        assert_eq!(cur.unwrap().val, 4);
        cur = cur.unwrap().next.as_ref();
        assert_eq!(cur.unwrap().val, 4);
        cur = cur.unwrap().next.as_ref();
        assert!(cur.is_none());
    }
    #[test]
    fn test_case_2() {
        let l1 = None;
        let l2 = None;

        assert_eq!(Solution::merge_two_lists(l1, l2), None);
    }
}

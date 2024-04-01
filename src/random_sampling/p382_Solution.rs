/**
382. 链表随机节点

给你一个单链表，随机选择链表的一个节点，并返回相应的节点值。每个节点 被选中的概率一样 。

实现 Solution 类：

Solution(ListNode head) 使用整数数组初始化对象。
int getRandom() 从链表中随机选择一个节点并返回该节点的值。链表中所有节点被选中的概率相等。


提示：

链表中的节点数在范围 [1, 104] 内
-104 <= Node.val <= 104
至多调用 getRandom 方法 104 次


进阶：

如果链表非常大且长度未知，该怎么处理？
你能否在不使用额外空间的情况下解决此问题？

https://leetcode.cn/problems/linked-list-random-node/description/
*/
use crate::utils::structs::ListNode;
use rand::{thread_rng, Rng};
///进阶：
///
///如果链表非常大且长度未知，该怎么处理？
///你能否在不使用额外空间的情况下解决此问题？
///
/// 本題給的內存限制蠻小的
///
/// 很基礎的方法可以把練表所有的地址遍歷一遍後記錄下來，即可快速取得值
/// 但這題是希望我們練習在內存無法全部讀取完資料的數據流情況下做隨機抽取
/// 水塘抽樣：https://zh.wikipedia.org/wiki/%E6%B0%B4%E5%A1%98%E6%8A%BD%E6%A8%A3
///

struct Solution {
    head: Option<Box<ListNode>>,
    vector: Vec<i32>,
}

impl Solution {
    pub fn new(head: Option<Box<ListNode>>) -> Self {
        let mut solution = Solution {
            head,
            vector: Vec::new(),
        };
        if let Some(ref mut head_ptr) = solution.head {
            solution.vector.push(head_ptr.val);
            solution.head = head_ptr.next.take();
        }
        solution
    }

    pub fn get_random(&mut self) -> i32 {
        let n_usize = self.vector.len();
        let mut rng = thread_rng();
        let index = rng.gen_range(0..n_usize);
        let mut ans = self.vector[index];

        // 練習水塘抽樣
        while let Some(ref mut head_ptr) = self.head.take() {
            self.vector.push(head_ptr.val);
            self.head = head_ptr.next.take();
            let prob = rng.gen_range(0..self.vector.len());
            if prob > n_usize {
                ans = self.vector[prob];
            }
        }

        ans
    }
}

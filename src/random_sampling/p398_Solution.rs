/**
398. 随机数索引

给你一个可能含有 重复元素 的整数数组 nums ，请你随机输出给定的目标数字 target 的索引。你可以假设给定的数字一定存在于数组中。

实现 Solution 类：

Solution(int[] nums) 用数组 nums 初始化对象。
int pick(int target) 从 nums 中选出一个满足 nums[i] == target 的随机索引 i 。如果存在多个有效的索引，则每个索引的返回概率应当相等。


提示：

1 <= nums.length <= 2 * 104
-231 <= nums[i] <= 231 - 1
target 是 nums 中的一个整数
最多调用 pick 函数 104 次

https://leetcode.cn/problems/random-pick-index/description/
*/
use rand::{thread_rng, Rng};
use std::collections::HashMap;

struct Solution {
    index_hashmap: HashMap<i32, Vec<usize>>,
}
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut solution = Solution {
            index_hashmap: HashMap::new(),
        };
        for i in 0..nums.len() {
            solution
                .index_hashmap
                .entry(nums[i])
                .or_insert(Vec::new())
                .push(i);
        }
        solution
    }

    fn pick(&self, target: i32) -> i32 {
        let mut rng = thread_rng();
        if let Some(vector) = self.index_hashmap.get(&target) {
            let s = rng.gen_range(0..vector.len());
            return vector[s] as i32;
        }
        -1
    }
}

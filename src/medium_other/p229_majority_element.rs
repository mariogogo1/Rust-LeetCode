/**
229. 多数元素 II

给定一个大小为 n 的整数数组，找出其中所有出现超过 ⌊ n/3 ⌋ 次的元素。

https://leetcode.cn/problems/majority-element-ii/description/
*/

pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        let n = (nums.len() / 3 + 1) as i32;
        let mut ans: Vec<i32> = Vec::new();
        for num in nums {
            let value = hashmap.entry(num).or_insert(0);
            *value += 1;
        }
        for (k, v) in hashmap {
            if v >= n {
                ans.push(k);
            }
        }

        ans
    }
}

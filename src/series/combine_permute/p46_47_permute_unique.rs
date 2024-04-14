/**
46. 全排列
47. 全排列 II

给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。

https://leetcode.cn/problems/permutations/description/
https://leetcode.cn/problems/permutations-ii/description/
*/
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    fn dfs(
        map: &mut HashMap<i32, i32>,
        keys: &Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        one_ans: &mut Vec<i32>,
        count: i32,
    ) {
        if count == 0 {
            ans.push(one_ans.clone());
            return;
        }

        for k in keys {
            if let Some(&val) = map.get(&k) {
                if val > 0 {
                    one_ans.push(*k);
                    *map.get_mut(k).unwrap() -= 1;
                    Self::dfs(map, keys, ans, one_ans, count - 1);
                    one_ans.pop();
                    *map.get_mut(k).unwrap() += 1;
                }
            }
        }
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len() as i32;
        let mut counts_hashmap: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *counts_hashmap.entry(num).or_insert(0) += 1;
        }
        let keys: Vec<i32> = counts_hashmap.keys().cloned().collect(); // 收集所有的鍵

        let mut ans = Vec::new();

        Self::dfs(&mut counts_hashmap, &keys, &mut ans, &mut Vec::new(), n);

        ans
    }
}

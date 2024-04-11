/**
491. 非递减子序列

给你一个整数数组 nums ，找出并返回所有该数组中不同的递增子序列，递增子序列中 至少有两个元素 。你可以按 任意顺序 返回答案。

数组中可能含有重复元素，如出现两个整数相等，也可以视作递增序列的一种特殊情况。

https://leetcode.cn/problems/non-decreasing-subsequences/description/
*/
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    fn add_new_vec(record_ans: &mut Vec<Vec<usize>>, j: usize, i: usize) {
        let mut new_vec: Vec<usize> = record_ans[j].clone();
        new_vec.push(i);
        record_ans.push(new_vec);
    }

    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut record_ans: Vec<Vec<usize>> = vec![vec![]];

        let mut index_hashmap: HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            let length = record_ans.len();
            for j in 0..length {
                if let Some(&idx) = record_ans[j].last() {
                    if nums[idx] < nums[i] {
                        if !index_hashmap.contains_key(&nums[i]) {
                            Self::add_new_vec(&mut record_ans, j, i);
                        } else if *index_hashmap.get(&nums[i]).unwrap_or(&0) < idx {
                            Self::add_new_vec(&mut record_ans, j, i);
                        }
                    } else if nums[idx] == nums[i]
                        && *index_hashmap.get(&nums[idx]).unwrap_or(&0) == idx
                    {
                        Self::add_new_vec(&mut record_ans, j, i);
                    }
                } else if !index_hashmap.contains_key(&nums[i]) {
                    Self::add_new_vec(&mut record_ans, j, i);
                }
            }
            index_hashmap.insert(nums[i], i);
        }

        let mut ans = Vec::new();
        for vec in record_ans.iter() {
            if vec.len() <= 1 {
                continue;
            }
            let mut one_ans = Vec::new();
            for &idx in vec {
                one_ans.push(nums[idx]);
            }
            ans.push(one_ans);
        }

        ans
    }
}

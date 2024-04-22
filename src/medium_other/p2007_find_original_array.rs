/**
2007. 从双倍数组中还原原数组

一个整数数组 original 可以转变成一个 双倍 数组 changed ，转变方式为将 original 中每个元素 值乘以 2 加入数组中，然后将所有元素 随机打乱 。

给你一个数组 changed ，如果 change 是 双倍 数组，那么请你返回 original数组，否则请返回空数组。original 的元素可以以 任意 顺序返回。

https://leetcode.cn/problems/find-original-array-from-doubled-array/description/
*/
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        changed.sort_unstable();
        let mut countmap = HashMap::new();
        let mut ans = Vec::new();
        let mut count = 0;

        for i in 0..changed.len() {
            if let Some(val) = countmap.get_mut(&(changed[i] / 2)) {
                if *val >= 1 && changed[i] % 2 == 0 {
                    count -= 1;
                    *val -= 1;
                    ans.push(changed[i] / 2);
                    continue;
                }
            }
            count += 1;
            *countmap.entry(changed[i]).or_insert(0) += 1;
        }

        if count == 0 {
            return ans;
        }
        vec![]
    }
}

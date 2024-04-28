/**
2453. 摧毁一系列目标

给你一个下标从 0 开始的数组 nums ，它包含若干正整数，表示数轴上你需要摧毁的目标所在的位置。同时给你一个整数 space 。

你有一台机器可以摧毁目标。给机器 输入 nums[i] ，这台机器会摧毁所有位置在 nums[i] + c * space 的目标，其中 c 是任意非负整数。你想摧毁 nums 中 尽可能多 的目标。

请你返回在摧毁数目最多的前提下，nums[i] 的 最小值 。

https://leetcode.cn/problems/destroy-sequential-targets/description/
*/
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let mut min_hash: HashMap<i32, i32> = HashMap::new();
        let mut count_hash: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            let x = num % space;

            *count_hash.entry(x).or_insert(0) += 1;
            if let Some(min_v) = min_hash.get_mut(&x) {
                if *min_v > num {
                    *min_v = num;
                }
            } else {
                min_hash.insert(x, num);
            }
        }

        let mut max_count: i32 = 0;
        let mut ans: i32 = i32::MAX;
        for (key, val) in count_hash {
            if max_count < val {
                max_count = val;
                ans = *min_hash.entry(key).or_insert(0);
            } else if max_count == val {
                ans = ans.min(*min_hash.entry(key).or_insert(0));
            }
        }

        ans
    }
}

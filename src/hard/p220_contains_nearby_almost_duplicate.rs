/**

220. 存在重复元素 III

给你一个整数数组 nums 和两个整数 indexDiff 和 valueDiff 。

找出满足下述条件的下标对 (i, j)：

i != j,
abs(i - j) <= indexDiff
abs(nums[i] - nums[j]) <= valueDiff
如果存在，返回 true ；否则，返回 false 。

https://leetcode.cn/problems/contains-duplicate-iii/description/
*/
pub struct Solution;

/// 桶排序
/// 把每個數字放進間隔大小為(value_diff+1)的桶子裡面
/// 如果一個桶子裡面有兩個元素 -> 間隔太小 return true
/// 還要特別檢查前後兩個桶子裡面是否有間隔過小的數字對
use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let value_diff = value_diff as i64;
        let i32_min_i64: i64 = i32::MIN as i64;

        let bucket_id = |x: &i64| (*x - i32_min_i64) / (value_diff + 1);

        let mut bucket_hashmap: HashMap<i64, i32> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let id = bucket_id(&(num as i64));
            if bucket_hashmap.contains_key(&id) {
                return true;
            } else {
                bucket_hashmap.insert(id, num);
            }
            if let Some(&w) = bucket_hashmap.get(&(id - 1)) {
                if (num - w) as i64 <= value_diff {
                    return true;
                }
            }
            if let Some(&w) = bucket_hashmap.get(&(id + 1)) {
                if (w - num) as i64 <= value_diff {
                    return true;
                }
            }
            if i >= index_diff as usize {
                let delete_id = bucket_id(&(nums[i - index_diff as usize] as i64));
                bucket_hashmap.remove(&delete_id);
            }
        }

        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0),
            true
        );
    }
    #[test]
    fn test_case() {}
}

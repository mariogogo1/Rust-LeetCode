/**
1224. 最大相等频率

给你一个正整数数组 nums，请你帮忙从该数组中找出能满足下面要求的 最长 前缀，并返回该前缀的长度：

从前缀中 恰好删除一个 元素后，剩下每个数字的出现次数都相同。
如果删除这个元素后没有剩余元素存在，仍可认为每个数字都具有相同的出现次数（也就是 0 次）。


提示：

2 <= nums.length <= 105
1 <= nums[i] <= 105

https://leetcode.cn/problems/maximum-equal-frequency/description/
*/
pub struct Solution;
use std::collections::HashMap;

/// 維護前綴數組的最大頻率max跟最小頻率min
/// 兩種情況可以滿足題意
/// 1.min + 1 = max && min*數字種類 +1 = 當前長度
/// 2.min = 1 && max*(數字種類-1) + 1 = 當前長度
/// 可以由哈希表紀錄 當前每個數字的頻率跟"數字種類"
/// 維護最小頻率 可以由另外一張哈希表維護

impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut fre_hashmap: HashMap<i32, i32> = HashMap::new();
        let mut min_fre_hashmap: HashMap<i32, i32> = HashMap::new();
        let mut max_fre = 0;
        let mut min_fre = 0;
        let mut ans: usize = 0;
        for i in 0..nums.len() {
            let value = fre_hashmap.entry(nums[i]).or_insert(0);
            *value += 1;
            max_fre = max_fre.max(*value);
            if *value == 1 {
                *min_fre_hashmap.entry(1).or_insert(0) += 1;
                min_fre = *value;
            } else {
                if let Some(s) = min_fre_hashmap.get_mut(&(*value - 1)) {
                    *s -= 1;
                    if *s == 0 && min_fre == *value - 1 {
                        min_fre += 1;
                    }
                }
                *min_fre_hashmap.entry(*value).or_insert(0) += 1;
            }

            if fre_hashmap.len() == 1 {
                ans = i + 1;
                continue;
            }

            if min_fre + 1 == max_fre && min_fre * fre_hashmap.len() as i32 == i as i32 {
                ans = i + 1;
                continue;
            }
            if min_fre == 1 && max_fre * (fre_hashmap.len() as i32 - 1) == i as i32 {
                ans = i + 1;
                continue;
            }
        }
        return ans as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::max_equal_freq(vec![1, 1, 1, 5, 2, 2, 2, 3, 3, 3, 4, 4, 4]),
            13
        );
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::max_equal_freq(vec![2, 2, 1, 1, 5, 3, 3, 5]), 7);
        assert_eq!(Solution::max_equal_freq(vec![1, 1]), 2);
    }
}

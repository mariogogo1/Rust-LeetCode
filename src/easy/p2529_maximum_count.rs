/**
2529. 正整数和负整数的最大计数

给你一个按 非递减顺序 排列的数组 nums ，返回正整数数目和负整数数目中的最大值。

换句话讲，如果 nums 中正整数的数目是 pos ，而负整数的数目是 neg ，返回 pos 和 neg二者中的最大值。
注意：0 既不是正整数也不是负整数。
https://leetcode.cn/problems/maximum-count-of-positive-integer-and-negative-integer/description/?envType=daily-question&envId=2024-04-09
*/

pub struct Solution;
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = nums.len();
        let mut neg_length: i32 = 0;
        let mut pos_length: i32 = 0;
        if nums[l] > 0 || nums[r - 1] < 0 {
            return nums.len() as i32;
        }
        // 這段二分查找需要思考一下，有點小麻煩
        while l < r {
            let m = (l + r) / 2;
            if nums[m] >= 0 {
                r = m;
            } else if nums[m] < 0 {
                l = m + 1;
            }
        }
        neg_length = l as i32;

        l = 0;
        r = nums.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            if nums[m] >= 1 {
                r = m;
            } else if nums[m] < 1 {
                l = m + 1;
            }
        }
        pos_length = (nums.len() - l) as i32;

        pos_length.max(neg_length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]), 3);
    }
}

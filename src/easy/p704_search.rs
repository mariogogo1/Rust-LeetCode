/**
704. 二分查找

给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target，

写一个函数搜索 nums 中的 target，如果目标值存在返回下标，否则返回 -1。

Hint:

你可以假设 nums 中的所有元素是不重复的。

n 将在 [1, 10000]之间。

nums 的每个元素都将在 [-9999, 9999]之间。

https://leetcode.cn/problems/binary-search/description/
*/

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // usize 為索引 https://doc.rust-lang.org/std/primitive.usize.html
        let mut lp: usize = 0;
        let mut rp: usize = nums.len();
        let mut mp;
        while lp < rp {
            mp = (lp + rp) / 2;

            if nums[mp] > target {
                rp = mp;
            } else if nums[mp] < target {
                lp = mp + 1;
            } else {
                return mp as i32;
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
    #[test]
    fn test_case() {}
}

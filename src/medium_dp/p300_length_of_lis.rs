/**
300. 最长递增子序列

给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。

子序列 是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7] 的
子序列
。
https://leetcode.cn/problems/longest-increasing-subsequence/description/
*/
/// 這題很關鍵，幾乎是遞增子序列的基礎題目，俄羅斯套娃信封這種經典題目就是從這裡延伸的
pub struct Solution;
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails = Vec::new();
        for &num in nums.iter() {
            let pos = tails.binary_search(&num).unwrap_or_else(|x| x);
            if pos == tails.len() {
                tails.push(num);
            } else {
                tails[pos] = num;
            }
        }
        tails.len() as i32
    }
}

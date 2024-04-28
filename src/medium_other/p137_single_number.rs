/**
137. 只出现一次的数字 II

给你一个整数数组 nums ，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次 。请你找出并返回那个只出现了一次的元素。

你必须设计并实现线性时间复杂度的算法且使用常数级空间来解决此问题。

https://leetcode.cn/problems/single-number-ii/description/
*/
pub struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..32 {
            let mut bits = 0;
            for &k in &nums {
                bits += (k >> i) & 1;
            }
            bits %= 3;
            ans |= bits << i;
        }

        ans
    }
}

/**
258. 各位相加

给定一个非负整数 num，反复将各个位上的数字相加，直到结果为一位数。返回这个结果。

https://leetcode.cn/problems/add-digits/description/
*/

pub struct Solution;
/// 數學性質 num%9 = num的個位數字相加%9
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num % 9 == 0 && num != 0 {
            return 9;
        }
        num % 9
    }
}

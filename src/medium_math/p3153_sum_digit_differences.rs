/**
3153. 所有数对中数位不同之和

车尔尼有一个数组 nums ，它只包含 正 整数，所有正整数的数位长度都 相同 。

两个整数的 数位不同 指的是两个整数 相同 位置上不同数字的数目。

请车尔尼返回 nums 中 所有 整数对里，数位不同之和。

https://leetcode.cn/problems/sum-of-digit-differences-of-all-pairs/description/
*/
struct Solution {}
impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let mut ans = 0 as i64;
        let mut count = vec![vec![0; 11]; 9];
        let mut digit = 0 as usize;
        {
            let mut the_num = nums[0];
            while the_num > 0 {
                digit += 1;
                the_num /= 10;
            }
        }
        for i in 0..nums.len() {
            let mut the_num = nums[i];
            for j in 0..digit {
                let s = (the_num % 10) as usize;
                ans += count[j][10] - count[j][s];
                count[j][10] += 1;
                count[j][s] += 1;
                the_num /= 10;
            }
        }
        ans
    }
}

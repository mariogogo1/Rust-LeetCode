/**
670. 最大交换

给定一个非负整数，你至多可以交换一次数字中的任意两位。返回你能得到的最大值。

给定数字的范围是 [0, 10^8]

https://leetcode.cn/problems/distribute-candies-among-children-ii/description/
*/
/// 因為範圍給定是10^8為上限，所以很多檢查可以不用作

pub struct Solution;

impl Solution {
    pub fn pow(mut x: i32, n: i32) -> i32 {
        for _ in 0..n {
            x *= 10;
        }
        return x;
    }
    pub fn maximum_swap(num: i32) -> i32 {
        let mut ans = num;
        let mut num_mut = num;
        let mut max_temp_index = -1;
        let mut max_index = -1;
        let mut change_index = -1;
        let mut count_digits = 0;
        let (mut max_temp, mut max, mut change) = (0, 0, 0);
        while num_mut > 0 {
            let x = num_mut % 10;
            if x > max_temp {
                max_temp = x;
                max_temp_index = count_digits;
            } else if x < max_temp {
                change = x;
                max = max_temp;
                change_index = count_digits;
                max_index = max_temp_index;
            }
            num_mut /= 10;
            count_digits += 1;
        }
        if change_index == -1 {
            return num;
        }
        ans += Self::pow(max - change, change_index);
        ans -= Self::pow(max - change, max_index);
        return ans;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::maximum_swap(9973), 9973);

        assert_eq!(Solution::maximum_swap(2736), 7236);
        assert_eq!(Solution::maximum_swap(98368), 98863);
    }
    #[test]
    fn test_case() {}
}

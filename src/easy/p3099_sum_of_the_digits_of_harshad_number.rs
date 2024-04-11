/**
3099. 哈沙德数

如果一个整数能够被其各个数位上的数字之和整除，则称之为 哈沙德数（Harshad number）。给你一个整数 x 。如果 x 是 哈沙德数 ，则返回 x 各个数位上的数字之和，否则，返回 -1 。

https://leetcode.cn/problems/harshad-number/description/
*/

pub struct Solution;
impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut num = x.clone();
        let mut ans = 0;
        while num > 0 {
            let digit = num % 10;
            ans += digit;
            num /= 10;
        }
        if x % ans == 0 {
            return ans;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(18), 9);
    }
    #[test]
    fn test_case() {}
}

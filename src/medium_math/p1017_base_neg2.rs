/**
1017. 负二进制转换

给你一个整数 n ，以二进制字符串的形式返回该整数的 负二进制（base -2）表示。

注意，除非字符串就是 "0"，否则返回的字符串中不能含有前导零。

https://leetcode.cn/problems/convert-to-base-2/description/?envType=daily-question&envId=2024-04-28
*/

pub struct Solution;
impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        let mut number: i32 = 0;
        let mut idx = 0;
        while n != 0 {
            let mut x = 0;
            if n > 0 {
                x = n - n / (-2) * (-2);
            } else if n < 0 {
                if n == n / (-2) * (-2) {
                    x = 0;
                } else {
                    x = n - (n / (-2) + 1) * (-2);
                }

                n -= x;
            }

            number |= x << idx;
            n /= (-2);
            idx += 1;
        }

        let binary_string = format!("{:b}", number);

        binary_string
    }
}

/**
6. Z 字形变换

将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。

比如输入字符串为 "PAYPALISHIRING" 行数为 3 时，排列如下：

P   A   H   N
A P L S I I G
Y   I   R
之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："PAHNAPLSIIGYIR"。

请你实现这个将字符串进行指定行数变换的函数：

string convert(string s, int numRows);

https://leetcode.cn/problems/zigzag-conversion/description/
*/

pub struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut ans = vec![0 as u8; s.len()];
        let mut index = 0;
        let n = 2 * (num_rows - 1);

        if num_rows == 1 {
            return s;
        }

        for i in (0..s.len()).step_by(n as usize) {
            ans[index] = s.as_bytes()[i];
            index += 1;
        }

        for row in 1..num_rows - 1 {
            let (mut true_jump, mut false_jump) = (2 * (num_rows - row - 1), 2 * row);
            let mut t_or_f = true;

            let mut i = row as usize;
            while i < s.len() {
                ans[index] = s.as_bytes()[i];
                index += 1;

                if !t_or_f {
                    i += false_jump as usize;
                } else {
                    i += true_jump as usize;
                }

                t_or_f = !t_or_f;
            }
        }

        for i in ((num_rows - 1) as usize..s.len()).step_by(n as usize) {
            ans[index] = s.as_bytes()[i];
            index += 1;
        }

        String::from_utf8(ans).unwrap()
    }
}

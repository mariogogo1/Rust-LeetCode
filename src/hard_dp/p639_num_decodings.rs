/**

639. 解码方法 II

一条包含字母 A-Z 的消息通过以下的方式进行了 编码 ：

'A' -> "1"
'B' -> "2"
...
'Z' -> "26"
要 解码 一条已编码的消息，所有的数字都必须分组，然后按原来的编码方案反向映射回字母（可能存在多种方式）。例如，"11106" 可以映射为：

"AAJF" 对应分组 (1 1 10 6)
"KJF" 对应分组 (11 10 6)
注意，像 (1 11 06) 这样的分组是无效的，因为 "06" 不可以映射为 'F' ，因为 "6" 与 "06" 不同。

除了 上面描述的数字字母映射方案，编码消息中可能包含 '*' 字符，可以表示从 '1' 到 '9' 的任一数字（不包括 '0'）。例如，编码字符串 "1*" 可以表示 "11"、"12"、"13"、"14"、"15"、"16"、"17"、"18" 或 "19" 中的任意一条消息。对 "1*" 进行解码，相当于解码该字符串可以表示的任何编码消息。

给你一个字符串 s ，由数字和 '*' 字符组成，返回 解码 该字符串的方法 数目 。

由于答案数目可能非常大，返回 109 + 7 的 模 。

https://leetcode.cn/problems/decode-ways-ii/description/
*/
pub struct Solution;
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let len = s.len();
        let s = s.as_bytes();
        let mut dp: Vec<i64> = vec![0; len];

        fn decode(kkk: &[u8]) -> i64 {
            match kkk.len() {
                1 => match kkk[0] {
                    b'*' => 9,
                    b'1'..=b'9' => 1,
                    _ => 0,
                },
                _ => match kkk[0] {
                    b'*' => match kkk[1] {
                        b'*' => 15,
                        b'0'..=b'6' => 2,
                        _ => 1,
                    },
                    b'1' => match kkk[1] {
                        b'*' => 9,
                        _ => 1,
                    },
                    b'2' => match kkk[1] {
                        b'*' => 6,
                        b'0'..=b'6' => 1,
                        _ => 0,
                    },
                    _ => 0,
                },
            }
        }

        for i in 0..len {
            let one_word = decode(&s[i..=i]);
            if i == 0 {
                dp[i] = one_word;
            } else {
                dp[i] = (one_word * dp[i - 1]) % 1_000_000_007;
            }

            if i > 0 {
                let two_words = decode(&s[i - 1..=i]);
                if i == 1 {
                    dp[i] = (dp[i] + two_words) % 1_000_000_007;
                } else {
                    dp[i] = (dp[i] + two_words * dp[i - 2]) % 1_000_000_007;
                }
            }
        }

        dp[len - 1] as i32
    }
}

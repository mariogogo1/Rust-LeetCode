/**
3138. 同位字符串连接的最小长度

给你一个字符串 s ，它由某个字符串 t 和它的 同位字符串 连接而成。

请你返回字符串 t 的 最小 可能长度。

同位字符串 指的是重新排列一个单词得到的另外一个字符串，原来字符串中的每个字符在新字符串中都恰好只使用一次。

https://leetcode.cn/problems/minimum-length-of-anagram-concatenation/description/
*/

pub struct Solution;
impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        let mut char_count = vec![0; 26]; // 26 个字母的计数数组
        let s: Vec<char> = s.chars().collect();

        for ch in &s {
            char_count[(*ch as u8 - b'a') as usize] += 1;
        }

        let mut the_greatest_factor = char_count[(s[0] as u8 - b'a') as usize];

        for &v in &char_count {
            the_greatest_factor = Self::gcd(the_greatest_factor, v);
        }

        'loop_1: for factor in 1..=the_greatest_factor {
            if the_greatest_factor % factor == 0 {
                let calculator = the_greatest_factor / factor;
                let ans = s.len() / calculator as usize;
                for i in 0..calculator {
                    let mut new_char_count = char_count
                        .iter()
                        .map(|&x| x / calculator)
                        .collect::<Vec<_>>();
                    for j in 0..ans {
                        let ch = (s[i * ans + j] as u8 - b'a') as usize;
                        char_count[ch] -= 1;
                        if char_count[ch] < 0 {
                            continue 'loop_1;
                        }
                    }
                }
                return ans as i32;
            }
        }

        s.len() as i32
    }

    pub fn gcd<T: std::cmp::PartialEq + std::ops::Rem<Output = T> + Default + Copy>(
        mut a: T,
        mut b: T,
    ) -> T {
        while b != T::default() {
            (a, b) = (b, a % b)
        }
        a
    }
}

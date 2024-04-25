/**
483. 最小好进制
以字符串的形式给出 n , 以字符串的形式返回 n 的最小 好进制  。

如果 n 的  k(k>=2) 进制数的所有数位全为1，则称 k(k>=2) 是 n 的一个 好进制 。

https://leetcode.cn/problems/smallest-good-base/description/
*/
pub struct Solution;

/// 先用 2 做為基底 看最長的11111...1111 會是多長=the_max_len
/// 從長度最長the_max_len 一直往回找的 長度只有2 的情況
/// 如果有任意的 x進制 11..111 = 10 進制的 n 立刻回傳 x
impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let n: Vec<char> = n.chars().collect();
        let mut num: i64 = 0;

        for i in 0..n.len() {
            num *= 10;
            num += (n[i] as u8 - b'0') as i64;
        }

        let mut x = num;
        let mut the_max_len = 0;
        while x > 0 {
            x -= 1;
            x >>= 1;
            the_max_len += 1;
        }

        // 找到該長度最有可能的解
        for i in (1..=the_max_len).rev() {
            let mut ans = Self::binary_search(i, num);
            if ans > 1 {
                return ans.to_string();
            }
        }

        unreachable!();
    }
    fn binary_search(i: i32, num: i64) -> i64 {
        let mut l: i64 = 2;
        let mut r: i64 = num;

        'loop1: while l < r {
            let m = l + (r - l) / 2;
            let mut calculate_num: i64 = 0;
            let mut calculate_i = i;
            while calculate_i >= 0 {
                if calculate_num.checked_mul(m).is_none() {
                    r = m;
                    continue 'loop1;
                }
                calculate_num *= m;
                calculate_num += 1;
                if calculate_num > num {
                    r = m;
                    continue 'loop1;
                }
                calculate_i -= 1;
            }
            if calculate_num == num {
                return m;
            } else if calculate_num < num {
                l = m + 1;
            }
        }

        0
    }
}

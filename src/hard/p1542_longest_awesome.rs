/**
1542. 找出最长的超赞子字符串

给你一个字符串 s 。请返回 s 中最长的 超赞子字符串 的长度。

「超赞子字符串」需满足满足下述两个条件：

该字符串是 s 的一个非空子字符串
进行任意次数的字符交换后，该字符串可以变成一个回文字符串

https://leetcode.cn/problems/find-longest-awesome-substring/description/
*/
pub struct Solution;
impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut count_map: Vec<usize> = vec![0; 1024]; // 用哈希表會很慢 數字數量一共是 2^10
        let mut ans = 0;

        let mut bitwise_count = 0 as usize;
        for (i, ch) in s.chars().enumerate() {
            let digit = ch.to_digit(10).unwrap();
            bitwise_count ^= 1 << digit; // 類似前綴和

            if count_map[bitwise_count] > 0 || bitwise_count == 0 {
                ans = ans.max(i + 1 - count_map[bitwise_count]);
            } else {
                count_map[bitwise_count] = i + 1;
            }

            for k in 0..10 {
                let idx = bitwise_count ^ (1 << k);
                if count_map[idx] > 0 || idx == 0 {
                    ans = ans.max(i + 1 - count_map[idx]);
                }
            }
        }

        ans as i32
    }
}

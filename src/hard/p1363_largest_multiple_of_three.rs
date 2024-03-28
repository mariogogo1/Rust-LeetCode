/**

1363. 形成三的最大倍数

给你一个整数数组 digits，你可以通过按 任意顺序 连接其中某些数字来形成 3 的倍数，请你返回所能得到的最大的 3 的倍数。

由于答案可能不在整数数据类型范围内，请以字符串形式返回答案。如果无法得到答案，请返回一个空字符串。返回的结果不应包含不必要的前导零。

提示：

1 <= digits.length <= 10^4
0 <= digits[i] <= 9

https://leetcode.cn/problems/largest-multiple-of-three/description/
*/
pub struct Solution;

/// 計算個數字MOD 0 1 2的數量，然後做數量判斷即可，小心一點就可以了

impl Solution {
    pub fn largest_multiple_of_three(nums: Vec<i32>) -> String {
        let mut digit_count_vec = vec![0; 10];
        let mut ans = Vec::new();
        let mut mod_zero_count: i32 = 0;
        let mut mod_one_count: i32 = 0;
        let mut mod_two_count: i32 = 0;
        for i in 0..nums.len() {
            let s = nums[i] as usize;
            digit_count_vec[s] += 1;
            if s % 3 == 0 {
                mod_zero_count += 1;
            } else if s % 3 == 1 {
                mod_one_count += 1;
            } else {
                mod_two_count += 1;
            }
        }

        let mut minus_mod_x = |x: usize| {
            if digit_count_vec[0 + x] > 0 {
                digit_count_vec[0 + x] -= 1;
            } else if digit_count_vec[3 + x] > 0 {
                digit_count_vec[3 + x] -= 1;
            } else {
                digit_count_vec[6 + x] -= 1;
            }
        };

        // 扣掉多餘的
        if (mod_one_count + mod_two_count * 2) % 3 == 1 {
            if mod_one_count % 3 == 0 {
                // 特殊處理
                if mod_one_count == 0 {
                    // -兩個mod 2 的
                    minus_mod_x(2);
                    minus_mod_x(2);
                } else {
                    minus_mod_x(1);
                }
            } else if mod_one_count % 3 == 1 {
                //-1 -4  -7
                minus_mod_x(1);
            } else {
                minus_mod_x(1);
            }
        } else if (mod_one_count + mod_two_count * 2) % 3 == 2 {
            if mod_one_count % 3 == 0 {
                //-2 -5  -8
                minus_mod_x(2);
            } else if mod_one_count % 3 == 1 {
                //-2 -5  -8
                minus_mod_x(2);
            } else {
                // 特殊處理
                if mod_two_count > 0 {
                    minus_mod_x(2);
                } else {
                    // -兩個mod 1 的
                    minus_mod_x(1);
                    minus_mod_x(1);
                }
            }
        }
        //println!("{:?}", digit_count_vec);
        //
        for i in (0..10).rev() {
            for _j in 0..digit_count_vec[i] {
                ans.push((i as u8 + b'0') as char);
            }
        }

        if ans.len() == digit_count_vec[0] && ans.len() > 0 {
            return "0".to_string();
        }
        // 前墜0 要刪除

        return ans.into_iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::largest_multiple_of_three(vec![8, 6, 7, 1, 0]),
            "8760".to_string()
        );
        assert_eq!(
            Solution::largest_multiple_of_three(vec![8, 1, 9]),
            "981".to_string()
        );
        assert_eq!(Solution::largest_multiple_of_three(vec![1]), "".to_string());
        assert_eq!(
            Solution::largest_multiple_of_three(vec![0, 0, 0, 0]),
            "0".to_string()
        );
    }
    #[test]
    fn test_case() {
        assert_eq!(
            Solution::largest_multiple_of_three(vec![5, 8]),
            "".to_string()
        );
    }
}

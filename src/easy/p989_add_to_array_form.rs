/**
989. 数组形式的整数加法

整数的 数组形式  num 是按照从左到右的顺序表示其数字的数组。

例如，对于 num = 1321 ，数组形式是 [1,3,2,1] 。
给定 num ，整数的 数组形式 ，和整数 k ，返回 整数 num + k 的 数组形式 。

https://leetcode.cn/problems/add-to-array-form-of-integer/description/
*/

pub struct Solution;
impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut next_one = false;

        for i in (0..num.len()).rev() {
            if k != 0 {
                num[i] += k % 10;
                if next_one {
                    num[i] += 1;
                }
                if num[i] >= 10 {
                    num[i] %= 10;
                    next_one = true;
                } else {
                    next_one = false;
                }
                k /= 10;
            } else {
                if next_one {
                    num[i] += 1;
                    if num[i] >= 10 {
                        num[i] %= 10;
                        next_one = true;
                    } else {
                        next_one = false;
                    }
                } else {
                    break;
                }
            }
        }
        let mut ans = Vec::new();
        if next_one {
            k += 1;
        }

        while k != 0 {
            let mut x = vec![k % 10];
            x.extend(ans).clone();
            ans = x;
            k /= 10;
        }

        if ans.is_empty() {
            return num;
        } else {
            ans.extend(num);
        }
        ans
    }
}

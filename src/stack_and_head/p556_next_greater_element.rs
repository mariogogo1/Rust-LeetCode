/**
556. 下一个更大元素 III

给你一个正整数 n ，请你找出符合条件的最小整数，其由重新排列 n 中存在的每位数字组成，并且其值大于 n 。如果不存在这样的正整数，则返回 -1 。

Hint:
注意 ，返回的整数应当是一个 32 位整数 ，如果存在满足题意的答案，但不是 32 位整数 ，同样返回 -1 。
1 <= n <= 231 - 1

https://leetcode.cn/problems/next-greater-element-ii/description/
*/

pub struct Solution;
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digit_stack: Vec<i32> = Vec::new();
        let mut n_mut = n;
        let mut changer = -1;
        while n_mut > 0 {
            let x = n_mut % 10;
            if let Some(&digit) = digit_stack.last() {
                if x < digit {
                    for (i, &num) in digit_stack.iter().enumerate() {
                        if num > x {
                            changer = digit_stack[i];
                            digit_stack[i] = x;
                            break;
                        }
                    }
                    break;
                }
            }
            digit_stack.push(x);
            n_mut /= 10;
        }
        if n_mut == 0 {
            return -1;
        }

        let mut ans = n_mut / 10; // 修改计算方式，简化代码逻辑
        ans = ans * 10 + changer; // 修改计算方式，简化代码逻辑
        for digit in digit_stack {
            if let Some(result) = ans.checked_mul(10).and_then(|x| x.checked_add(digit)) {
                ans = result;
            } else {
                return -1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::next_greater_element(12), 21);
        assert_eq!(Solution::next_greater_element(231), 312);
        assert_eq!(Solution::next_greater_element(21), -1);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::next_greater_element(2147483486), -1);
        assert_eq!(Solution::next_greater_element(1999999999), -1);
    }
}

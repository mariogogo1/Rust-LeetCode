/**
972. 相等的有理数

给定两个字符串 s 和 t ，每个字符串代表一个非负有理数，只有当它们表示相同的数字时才返回 true 。字符串中可以使用括号来表示有理数的重复部分。

有理数 最多可以用三个部分来表示：整数部分 <IntegerPart>、小数非重复部分 <NonRepeatingPart> 和小数重复部分 <(><RepeatingPart><)>。数字可以用以下三种方法之一来表示：

<IntegerPart>
例： 0 ,12 和 123
<IntegerPart><.><NonRepeatingPart>
例： 0.5 , 1. , 2.12 和 123.0001
<IntegerPart><.><NonRepeatingPart><(><RepeatingPart><)>
例： 0.1(6) ， 1.(9)， 123.00(1212)
十进制展开的重复部分通常在一对圆括号内表示。例如：

1 / 6 = 0.16666666... = 0.1(6) = 0.1666(6) = 0.166(66)

提示：

每个部分仅由数字组成。
整数部分 <IntegerPart> 不会以零开头。（零本身除外）
1 <= <IntegerPart>.length <= 4
0 <= <NonRepeatingPart>.length <= 4
1 <= <RepeatingPart>.length <= 4

https://leetcode.cn/problems/equal-rational-numbers/description/
*/
pub struct Solution;
/// 把字符串分成 整數 分子 分母 三個部分
/// 分子跟分母要進位成為整數，使得分子 < 分母，
/// 進行比較 整數部分必須相等 小數部分：分子分母交叉相乘即可 判斷是否相等
/// 循環小數的分子分母計算： https://zh.wikipedia.org/wiki/%E5%BE%AA%E7%8E%AF%E5%B0%8F%E6%95%B0

impl Solution {
    fn string2rational(s: &str) -> (i64, i64, i64) {
        let mut integer: i64 = 0;

        let mut numerator: i64 = 0;
        let mut non_recurring_numerator: i64 = 0;

        let mut denominator: i64 = 1;
        let mut recurring_count: i64 = 0;

        let mut is_decimal = false;
        let mut is_recurring = false;

        for ch in s.chars() {
            if ch == '.' {
                is_decimal = true;
                continue;
            } else if ch == '(' {
                is_recurring = true;
                continue;
            } else if ch == ')' {
                break;
            }
            let x = ch as i64 - '0' as i64;
            if is_decimal {
                numerator = numerator * 10 + x;
                if is_recurring {
                    recurring_count += 1;
                } else {
                    non_recurring_numerator = non_recurring_numerator * 10 + x;
                    denominator *= 10;
                }
            } else {
                integer = integer * 10 + x;
            }
        }
        let mut recurring_denominator: i64 = 1;
        for _ in 0..recurring_count {
            recurring_denominator *= 10;
        }

        if is_recurring {
            if numerator - non_recurring_numerator >= denominator * (recurring_denominator - 1) {
                return (
                    integer + 1,
                    numerator - non_recurring_numerator - denominator * (recurring_denominator - 1),
                    denominator * (recurring_denominator - 1),
                );
            }
            return (
                integer,
                numerator - non_recurring_numerator,
                denominator * (recurring_denominator - 1),
            );
        }
        return (integer, numerator, denominator);
    }

    pub fn is_rational_equal(s: String, t: String) -> bool {
        let (integer1, numerator1, denominator1) = Self::string2rational(&s);
        let (integer2, numerator2, denominator2) = Self::string2rational(&t);
        if integer1 == integer2 && numerator1 * denominator2 == denominator1 * numerator2 {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            // special case
            Solution::is_rational_equal("0.9(9)".to_string(), "1.".to_string()),
            true
        );
    }
    #[test]
    fn test_case() {
        assert_eq!(
            Solution::is_rational_equal("0.(52)".to_string(), "0.5(25)".to_string()),
            true
        );
        assert_eq!(
            Solution::is_rational_equal("0.1666(6)".to_string(), "0.166(66)".to_string()),
            true
        );
        assert_eq!(
            // special case
            Solution::is_rational_equal("0.9(9)".to_string(), "1.1".to_string()),
            false
        );
    }
}

/**
3100. 换水问题 II

给你两个整数 numBottles 和 numExchange 。

numBottles 代表你最初拥有的满水瓶数量。在一次操作中，你可以执行以下操作之一：

喝掉任意数量的满水瓶，使它们变成空水瓶。
用 numExchange 个空水瓶交换一个满水瓶。然后，将 numExchange 的值增加 1 。
注意，你不能使用相同的 numExchange 值交换多批空水瓶。例如，如果 numBottles == 3 并且 numExchange == 1 ，则不能用 3 个空水瓶交换成 3 个满水瓶。

返回你 最多 可以喝到多少瓶水。

https://leetcode.cn/problems/water-bottles-ii/description/
*/

pub struct Solution;

impl Solution {
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut empty_bottles = num_bottles;
        let mut ans = num_bottles;
        num_bottles = 0;
        while empty_bottles >= num_exchange {
            while empty_bottles >= num_exchange {
                empty_bottles -= num_exchange;
                num_bottles += 1;
                num_exchange += 1;
            }
            empty_bottles += num_bottles;
            ans += num_bottles;
            num_bottles = 0;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case() {}
}

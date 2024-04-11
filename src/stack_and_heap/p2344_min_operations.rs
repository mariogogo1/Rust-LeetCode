/**
2344. 使数组可以被整除的最少删除次数

给你两个正整数数组 nums 和 numsDivide 。你可以从 nums 中删除任意数目的元素。

请你返回使 nums 中 最小 元素可以整除 numsDivide 中所有元素的 最少 删除次数。如果无法得到这样的元素，返回 -1 。

如果 y % x == 0 ，那么我们说整数 x 整除 y 。

https://leetcode.cn/problems/minimum-deletions-to-make-array-divisible/description/
*/

pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let mut greatest_factor = nums_divide[0];
        for num in nums_divide {
            greatest_factor = Self::gcd(greatest_factor, num);
        }

        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for num in nums {
            min_heap.push(Reverse(num));
        }

        let mut ans = 0;
        let mut hashset = HashSet::new();
        while let Some(Reverse(num)) = min_heap.peek() {
            if *num > greatest_factor {
                break;
            }
            if hashset.contains(num) {
                min_heap.pop();
                ans += 1;
                continue;
            }
            if greatest_factor % *num == 0 {
                return ans;
            } else {
                hashset.insert(*num);
                min_heap.pop();
                ans += 1;
            }
        }

        -1
    }

    fn gcd<T: std::cmp::PartialEq + std::ops::Rem<Output = T> + Default + Copy>(
        mut a: T,
        mut b: T,
    ) -> T {
        while b != T::default() {
            (a, b) = (b, a % b)
        }
        a
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

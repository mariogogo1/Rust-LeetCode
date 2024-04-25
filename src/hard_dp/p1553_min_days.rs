/**
1553. 吃掉 N 个橘子的最少天数

厨房里总共有 n 个橘子，你决定每一天选择如下方式之一吃这些橘子：

吃掉一个橘子。
如果剩余橘子数 n 能被 2 整除，那么你可以吃掉 n/2 个橘子。
如果剩余橘子数 n 能被 3 整除，那么你可以吃掉 2*(n/3) 个橘子。
每天你只能从以上 3 种方案中选择一种方案。

请你返回吃掉所有 n 个橘子的最少天数。

提示：

1 <= n <= 2*10^9

https://leetcode.cn/problems/minimum-number-of-days-to-eat-n-oranges/description/
*/
pub struct Solution;
//1553. 吃掉 N 个橘子的最少天数
use std::collections::HashMap;

impl Solution {
    pub fn min_days(n: i32) -> i32 {
        let mut oranges_hash: HashMap<i32, i32> = HashMap::new();
        let mut oranges_vec: Vec<i32> = vec![n];
        let mut index: usize = 0;
        oranges_hash.insert(n, 0);
        while index < oranges_vec.len() {
            let i = oranges_vec[index];
            if let Some(&x) = oranges_hash.get(&i) {
                if i == 0 {
                    return x;
                }
                if !oranges_hash.contains_key(&(i - 1)) {
                    oranges_hash.insert(i - 1, x + 1);
                    oranges_vec.push(i - 1);
                }
                if i % 2 == 0 {
                    if !oranges_hash.contains_key(&(i / 2)) {
                        oranges_hash.insert(i / 2, x + 1);
                        oranges_vec.push(i / 2);
                    }
                }

                if i % 3 == 0 {
                    if !oranges_hash.contains_key(&(i / 3)) {
                        oranges_hash.insert(i / 3, x + 1);
                        oranges_vec.push(i / 3);
                    }
                }
            }
            index += 1;
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::min_days(10), 4);
        assert_eq!(Solution::min_days(1), 1);
        assert_eq!(Solution::min_days(3), 2);
        assert_eq!(Solution::min_days(56), 6);
    }
    #[test]
    fn test_case() {}
}

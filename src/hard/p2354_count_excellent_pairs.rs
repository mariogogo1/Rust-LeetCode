/**

2354. 优质数对的数目

给你一个下标从 0 开始的正整数数组 nums 和一个正整数 k 。

如果满足下述条件，则数对 (num1, num2) 是 优质数对 ：

num1 和 num2 都 在数组 nums 中存在。
num1 OR num2 和 num1 AND num2 的二进制表示中值为 1 的位数之和大于等于 k ，其中 OR 是按位 或 操作，而 AND 是按位 与 操作。
返回 不同 优质数对的数目。

如果 a != c 或者 b != d ，则认为 (a, b) 和 (c, d) 是不同的两个数对。例如，(1, 2) 和 (2, 1) 不同。

注意：如果 num1 在数组中至少出现 一次 ，则满足 num1 == num2 的数对 (num1, num2) 也可以是优质数对。

https://leetcode.cn/problems/number-of-excellent-pairs/description/
*/
pub struct Solution;

/// 集合性質
/// num[i].count_ones() +  num[j].count_ones() = AND(num[i],num[j]).count_ones() + OR(num[i],num[j]).count_ones()
use std::collections::HashSet;
impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let mut count_ones_vec = vec![0; 33];
        let mut num_hashset = HashSet::new();
        let mut total = 0 as i64;

        for num in nums {
            if num_hashset.contains(&num) {
                continue;
            }
            num_hashset.insert(num);
            let index = num.count_ones() as usize;
            count_ones_vec[index] += 1;
            total += 1;
        }

        let k = k as usize;
        let mut ans = 0 as i64;
        for i in 0..33 {
            if k >= i {
                let mut s = 0 as i64;
                for j in (k - i)..33 {
                    s += count_ones_vec[j];
                }
                ans += s * count_ones_vec[i];
            } else {
                ans += total * count_ones_vec[i];
            }
        }

        ans
    }
}

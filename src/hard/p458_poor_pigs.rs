/**

2561. 重排水果

你有两个果篮，每个果篮中有 n 个水果。给你两个下标从 0 开始的整数数组 basket1 和 basket2 ，用以表示两个果篮中每个水果的交换成本。你想要让两个果篮相等。为此，可以根据需要多次执行下述操作：

选中两个下标 i 和 j ，并交换 basket1 中的第 i 个水果和 basket2 中的第 j 个水果。
交换的成本是 min(basket1i,basket2j) 。
根据果篮中水果的成本进行排序，如果排序后结果完全相同，则认为两个果篮相等。

返回使两个果篮相等的最小交换成本，如果无法使两个果篮相等，则返回 -1 。

提示：

basket1.length == bakste2.length
1 <= basket1.length <= 105
1 <= basket1i,basket2i <= 109

https://leetcode.cn/problems/rearranging-fruits/description/
*/
pub struct Solution;

/// 紀錄兩籃的水果，做以下操作：
/// 紀錄第一籃和第二籃每種水果總數，兩籃有一樣的部分就共同消除
/// 兩籃剩下的部分合併，若剩下的水果有某種類為單數個，返回-1
/// 剩下各種水果數量必定偶數，合併之後個數量/2就是最後的果籃部分，
/// ** 特殊交換操作：使用兩個果籃裡面最小的數字x當工具，"可能"會比兩兩交患的成本低 min(2*x,a,b)
///     
use std::collections::HashMap;

impl Solution {
    fn min(a: i32, b: i32, c: i32) -> i32 {
        a.min(b).min(c)
    }

    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut basket1_hashmap: HashMap<i32, i32> = HashMap::new();
        let mut basket2_hashmap: HashMap<i32, i32> = HashMap::new();
        let mut min_fruit = i32::MAX;

        for fruit in basket1 {
            min_fruit = min_fruit.min(fruit);
            *basket1_hashmap.entry(fruit).or_insert(0) += 1;
        }
        for fruit in basket2 {
            min_fruit = min_fruit.min(fruit);
            if let Some(s) = basket1_hashmap.get_mut(&fruit) {
                if *s > 0 {
                    *s -= 1;
                } else {
                    *basket2_hashmap.entry(fruit).or_insert(0) += 1;
                }
            } else {
                *basket2_hashmap.entry(fruit).or_insert(0) += 1;
            }
        }

        // 將basket1_hashmap的KEY由大排到小，VALUE為0的跳過
        let mut basket1_vec = Vec::new();
        for (fruit, counts) in basket1_hashmap {
            if counts % 2 == 1 {
                return -1;
            }
            for _ in 0..counts / 2 {
                basket1_vec.push(fruit);
            }
        }
        basket1_vec.sort_by(|x, y| x.cmp(&y));

        // 將basket2_hashmap的KEY由小排到大

        let mut basket2_vec = Vec::new();
        for (fruit, counts) in basket2_hashmap {
            if counts % 2 == 1 {
                return -1;
            }
            for _ in 0..counts / 2 {
                basket2_vec.push(fruit);
            }
        }

        basket2_vec.sort_by(|x, y| y.cmp(&x));

        let mut ans: i64 = 0;
        for i in 0..basket1_vec.len() {
            ans += Self::min(basket1_vec[i], basket2_vec[i], 2 * min_fruit) as i64;
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::min_cost(vec![1, 4, 1, 2], vec![4, 2, 2, 2]), 1);
        assert_eq!(Solution::min_cost(vec![1, 4, 1, 2], vec![4, 2, 2, 5]), -1);
    }
    #[test]
    fn test_case() {}
}

/**
1345. 跳跃游戏 IV

给你一个整数数组 arr ，你一开始在数组的第一个元素处（下标为 0）。

每一步，你可以从下标 i 跳到下标 i + 1 、i - 1 或者 j ：

i + 1 需满足：i + 1 < arr.length
i - 1 需满足：i - 1 >= 0
j 需满足：arr[i] == arr[j] 且 i != j
请你返回到达数组最后一个元素的下标处所需的 最少操作次数 。

注意：任何时候你都不能跳到数组外面。

https://leetcode.cn/problems/jump-game-iv/description/
*/
// 可以用維護抵達每個位置的最少步數 來優化
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut hash_map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (index, &node) in arr.iter().enumerate() {
            hash_map.entry(node).or_insert(Vec::new()).push(index);
        }

        let mut visited: HashSet<usize> = HashSet::new();
        let mut arr_vec: Vec<usize> = vec![0];
        let mut step = 0;
        visited.insert(0);

        while !arr_vec.is_empty() {
            let go_vec = arr_vec.clone();
            arr_vec.clear();

            for &node in &go_vec {
                if node == arr.len() - 1 {
                    return step;
                }

                if let Some(indexes) = hash_map.get(&arr[node]) {
                    arr_vec.extend(indexes);
                    hash_map.remove(&arr[node]);
                }

                if node + 1 < arr.len()
                    && !visited.contains(&(node + 1))
                    && hash_map.contains_key(&arr[node + 1])
                {
                    arr_vec.push(node + 1);
                    visited.insert(node + 1);
                }

                if node > 0
                    && !visited.contains(&(node - 1))
                    && hash_map.contains_key(&arr[node - 1])
                {
                    arr_vec.push(node - 1);
                    visited.insert(node - 1);
                }
            }

            step += 1;
        }

        step
    }
}

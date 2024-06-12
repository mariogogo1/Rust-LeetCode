/**
765. 情侣牵手

n 对情侣坐在连续排列的 2n 个座位上，想要牵到对方的手。

人和座位由一个整数数组 row 表示，其中 row[i] 是坐在第 i 个座位上的人的 ID。情侣们按顺序编号，第一对是 (0, 1)，第二对是 (2, 3)，以此类推，最后一对是 (2n-2, 2n-1)。

返回 最少交换座位的次数，以便每对情侣可以并肩坐在一起。 每次交换可选择任意两人，让他们站起来交换座位。

https://leetcode.cn/problems/couples-holding-hands/description/
*/

pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let mut seat_map: HashMap<i32, Vec<usize>> = HashMap::new();
        let n = row.len() / 2;
        let mut visited = vec![false; n];
        let mut ring_count = 0;

        for (key, &value) in row.iter().enumerate() {
            let couple = value / 2;
            if let Some(seats) = seat_map.get_mut(&couple) {
                seats.push(key / 2);
            } else {
                seat_map.insert(couple, vec![key / 2]);
            }
        }

        for i in 0..n {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            let mut seat_now = i;
            let mut b = row[2 * i] / 2;
            let mut g = row[2 * i + 1] / 2;

            while b != g {
                let seats = seat_map.get(&g).unwrap();
                if seat_now == seats[0] {
                    visited[seats[1]] = true;
                    seat_now = seats[1];
                    if row[seats[1] * 2] / 2 == g {
                        g = row[seats[1] * 2 + 1] / 2;
                    } else {
                        g = row[seats[1] * 2] / 2;
                    }
                } else {
                    visited[seats[0]] = true;
                    seat_now = seats[0];
                    if row[seats[0] * 2] / 2 == g {
                        g = row[seats[0] * 2 + 1] / 2;
                    } else {
                        g = row[seats[0] * 2] / 2;
                    }
                }
            }
            ring_count += 1;
        }

        (row.len() / 2 - ring_count) as i32
    }
}

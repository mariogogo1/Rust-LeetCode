/**
881. 救生艇

给定数组 people 。people[i]表示第 i 个人的体重 ，船的数量不限，每艘船可以承载的最大重量为 limit。

每艘船最多可同时载两人，但条件是这些人的重量之和最多为 limit。

返回 承载所有人所需的最小船数 。

https://leetcode.cn/problems/boats-to-save-people/description/
*/
pub struct Solution;
impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as usize;
        let mut ans = 0;
        let mut hash = vec![0; limit + 1];
        for person in people {
            hash[person as usize] += 1;
        }
        let mut r = (limit + 1) / 2;
        let mut l = limit / 2;
        let mid = l + 1;
        if l == r {
            ans += hash[r] / 2;
            hash[r] -= ans * 2;
            l -= 1;
        }

        while l > 0 && r < limit {
            if l + r >= limit {
                l = limit - r;
            }

            if hash[r] < hash[l] {
                hash[l] -= hash[r];
                ans += hash[r];
                hash[r] = 0;
                r += 1;
            } else {
                hash[r] -= hash[l];
                ans += hash[l];
                hash[l] = 0;
                l -= 1;
            }
        }

        for i in mid..=limit {
            ans += hash[i];
        }

        let mut total = 0;
        for i in 0..mid {
            total += hash[i];
        }

        ans += (total + 1) / 2;

        ans
    }
}

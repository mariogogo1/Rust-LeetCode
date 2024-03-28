/**
403. 青蛙过河

一只青蛙想要过河。 假定河流被等分为若干个单元格，并且在每一个单元格内都有可能放有一块石子（也有可能没有）。 青蛙可以跳上石子，但是不可以跳入水中。

给你石子的位置列表 stones（用单元格序号 升序 表示）， 请判定青蛙能否成功过河（即能否在最后一步跳至最后一块石子上）。开始时， 青蛙默认已站在第一块石子上，并可以假定它第一步只能跳跃 1 个单位（即只能从单元格 1 跳至单元格 2 ）。

如果青蛙上一步跳跃了 k 个单位，那么它接下来的跳跃距离只能选择为 k - 1、k 或 k + 1 个单位。 另请注意，青蛙只能向前方（终点的方向）跳跃。

2 <= stones.length <= 2000
0 <= stones[i] <= 231 - 1
stones[0] == 0
stones 按严格升序排列

https://leetcode.cn/problems/frog-jump/description/
*/
pub struct Solution;

use std::collections::HashMap;

/// 關鍵想法：
/// 如果只能跳n次，最遠的單次跳躍距離是n，
/// 如果有連續兩塊石頭的距離超過該次最遠跳躍，就不可能跳到終點。
/// 有n塊石頭，最多的跳躍次數是(n-1)次，跳躍距離最多有[0~(n-1)]，一共n種可能 ，所以可以做n個項目紀錄。
/// --> 轉移矩陣 dp[i][distance] = dp[j][distance+1] || dp[j][distance] ||dp[j][distance-1]
/// distance =  nums[i] -nums[j]
/// 用j的跳躍距離跳到nums[i]上

impl Solution {
    pub fn can_cross(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let mut dp = vec![vec![false; n]; n];
        dp[0][0] = true;

        for i in 1..n {
            for j in (0..i).rev() {
                let distance = (nums[i] - nums[j]) as usize;
                //  如果只能跳i次，最遠的單次跳躍距離是i，
                if distance > j + 1 {
                    break;
                }

                // 嚴格遞增 distance >= 1
                dp[i][distance] = dp[j][distance - 1] || dp[j][distance] || dp[j][distance + 1];
                if i == n - 1 && dp[i][distance] {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn can_cross_notgood(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut pos_hashmap: HashMap<i32, usize> = HashMap::with_capacity(n);
        let mut jump_distance_vec: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in 0..n {
            pos_hashmap.insert(nums[i], i);
        }
        jump_distance_vec[0].push(0);
        if nums.len() == 1 {
            return true;
        } else if nums[1] >= 2 {
            return false;
        }
        jump_distance_vec[1].push(1);

        for i in 1..n - 1 {
            let mut jump_distance: HashMap<i32, bool> = HashMap::new();
            let s = jump_distance_vec[i].len();
            for j in 0..s {
                for delta in (-1)..=1 {
                    let x: i32 = jump_distance_vec[i][j] as i32 + delta;
                    if x > 0 && !jump_distance.contains_key(&x) {
                        if let Some(&index) = pos_hashmap.get(&(x + nums[i])) {
                            jump_distance_vec[index].push(x as usize);
                            jump_distance.insert(x, true);
                        }
                    }
                }
            }
        }

        return !jump_distance_vec[n - 1].is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]), true);
        assert_eq!(Solution::can_cross(vec![0, 1, 2, 3, 4, 8, 9, 11]), false);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::can_cross(vec![0, 2]), false);
    }
}

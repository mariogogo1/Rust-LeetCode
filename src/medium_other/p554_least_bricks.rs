/**
554. 砖墙

你的面前有一堵矩形的、由 n 行砖块组成的砖墙。这些砖块高度相同（也就是一个单位高）但是宽度不同。每一行砖块的宽度之和相等。

你现在要画一条 自顶向下 的、穿过 最少 砖块的垂线。如果你画的线只是从砖块的边缘经过，就不算穿过这块砖。你不能沿着墙的两个垂直边缘之一画线，这样显然是没有穿过一块砖的。

给你一个二维数组 wall ，该数组包含这堵墙的相关信息。其中，wall[i] 是一个代表从左至右每块砖的宽度的数组。你需要找出怎样画才能使这条线 穿过的砖块数量最少 ，并且返回 穿过的砖块数量 。

提示：

n == wall.length
1 <= n <= 104
1 <= wall[i].length <= 104
1 <= sum(wall[i].length) <= 2 * 104
对于每一行 i ，sum(wall[i]) 是相同的
1 <= wall[i][j] <= 231 - 1

https://leetcode.cn/problems/brick-wall/description/
*/
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let n = wall.len();
        let mut width = 0;
        let mut gap_hashmap: HashMap<i32, i32> = HashMap::new();
        let mut max_gap: i32 = 0;
        for i in 0..n {
            for brick in &wall[i] {
                width += brick;
                *gap_hashmap.entry(width).or_insert(0) += 1;
            }
            *gap_hashmap.get_mut(&width).unwrap() -= 1;
            width = 0;
        }
        for (_, v) in gap_hashmap {
            max_gap = max_gap.max(v);
        }

        n as i32 - max_gap
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::least_bricks(vec![
                vec![1, 2, 2, 1],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![2, 4],
                vec![3, 1, 2],
                vec![1, 3, 1, 1]
            ]),
            2
        );
        assert_eq!(Solution::least_bricks(vec![vec![1], vec![1], vec![1]]), 3);
    }
    #[test]
    fn test_case() {}
}

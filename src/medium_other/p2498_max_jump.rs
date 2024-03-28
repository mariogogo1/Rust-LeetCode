/**
2498. 青蛙过河 II

给你一个下标从 0 开始的整数数组 stones ，数组中的元素 严格递增 ，表示一条河中石头的位置。

一只青蛙一开始在第一块石头上，它想到达最后一块石头，然后回到第一块石头。同时每块石头 至多 到达 一次。

一次跳跃的 长度 是青蛙跳跃前和跳跃后所在两块石头之间的距离。

更正式的，如果青蛙从 stones[i] 跳到 stones[j] ，跳跃的长度为 |stones[i] - stones[j]| 。
一条路径的 代价 是这条路径里的 最大跳跃长度 。

请你返回这只青蛙的 最小代价 。

Hint:
2 <= stones.length <= 105
0 <= stones[i] <= 109
stones[0] == 0
stones 中的元素严格递增。

https://leetcode.cn/problems/frog-jump-ii/description/
*/
pub struct Solution;
// 青蛙II
/// 1.要讓Max(代價)最小化，跳躍策略：每次跳躍跨過得石頭數量要少--> 跨過0顆石頭產生的代價 小於  跨過1顆石頭產生的代價
/// 去程時：跨過n顆石頭產生的代價 小於  跨過n+1顆石頭產生的代價
/// 2.考慮回程時產生的代價 -->  跨過0顆石頭產生的代價 大於  跨過1顆石頭產生的代價
///
/// 石頭：a b1 b2 c
/// a -> b1 -> b2 -> c -> a
/// b1 到 b2 的相鄰跳躍使得 回程的 c 到 a 跨過2顆石頭 ，此相鄰跳躍的代價必定 > 間隔跳躍的代價
/// 間隔跳躍：a -> b2 -> c -> b1 -> a
///
/// 3.總結：去程時，若有跳躍跨過0顆石頭(相鄰跳躍) == 回程時有跳躍跨過2顆石頭以上
/// -->並非最佳策略
/// 接著考慮 間隔一顆石頭跳躍 --> 回程跟去程皆可產生最佳跳躍策略(無法更優化)
///  

impl Solution {
    pub fn max_jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 2 {
            return nums[1] - nums[0];
        }
        let mut cost = nums[2] - nums[0];
        for i in 2..nums.len() {
            cost = cost.max(nums[i] - nums[i - 2]);
        }
        return cost;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::max_jump(vec![0, 2, 5, 6, 7]), 5);
    }
    #[test]
    fn test_case() {}
}

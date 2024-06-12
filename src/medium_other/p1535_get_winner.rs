/**
1535. 找出数组游戏的赢家

给你一个由 不同 整数组成的整数数组 arr 和一个整数 k 。

每回合游戏都在数组的前两个元素（即 arr[0] 和 arr[1] ）之间进行。比较 arr[0] 与 arr[1] 的大小，较大的整数将会取得这一回合的胜利并保留在位置 0 ，较小的整数移至数组的末尾。当一个整数赢得 k 个连续回合时，游戏结束，该整数就是比赛的 赢家 。

返回赢得比赛的整数。

题目数据 保证 游戏存在赢家。

https://leetcode.cn/problems/find-the-winner-of-an-array-game/description/
*/
pub struct Solution;
impl Solution {
    pub fn get_winner(mut arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let mut idx = 0 as usize;
        let mut count = k;
        let mut max_v = arr[0];

        loop {
            if max_v < arr[idx + 1] {
                max_v = arr[idx + 1];
                count = k;
            }
            count -= 1;

            idx += 1;
            if count == 0 || idx >= n - 1 {
                return max_v;
            }
        }
        unreachable!()
    }
}

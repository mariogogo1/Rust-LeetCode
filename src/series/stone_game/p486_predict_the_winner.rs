/**
486. 预测赢家

给你一个整数数组 nums 。玩家 1 和玩家 2 基于这个数组设计了一个游戏。

玩家 1 和玩家 2 轮流进行自己的回合，玩家 1 先手。开始时，两个玩家的初始分值都是 0 。每一回合，玩家从数组的任意一端取一个数字（即，nums[0] 或 nums[nums.length - 1]），取到的数字将会从数组中移除（数组长度减 1 ）。玩家选中的数字将会加到他的得分上。当数组中没有剩余数字可取时，游戏结束。

如果玩家 1 能成为赢家，返回 true 。如果两个玩家得分相等，同样认为玩家 1 是游戏的赢家，也返回 true 。你可以假设每个玩家的玩法都会使他的分数最大化。

https://leetcode.cn/problems/predict-the-winner/description/
*/

/// a 先手選擇n個數字時 = max(選左邊+ b後手選擇右邊n-1個數字 ,選右邊+ b後手選擇左邊n-1個數字)
/// b 就是選a 剩下的
impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp_a: Vec<Vec<i32>> = vec![vec![0; n]; n];
        let mut dp_b: Vec<Vec<i32>> = vec![vec![0; n]; n];

        for i in 0..nums.len() {
            dp_a[i][i] = nums[i];
        }

        for i in (0..(n - 1)).rev() {
            for j in (i + 1)..n {
                if (nums[i] + dp_b[i + 1][j]) >= (nums[j] + dp_b[i][j - 1]) {
                    dp_a[i][j] = nums[i] + dp_b[i + 1][j];
                    dp_b[i][j] = dp_a[i + 1][j];
                } else {
                    dp_a[i][j] = nums[j] + dp_b[i][j - 1];
                    dp_b[i][j] = dp_a[i][j - 1];
                }
            }
        }

        dp_a[0][n - 1] >= dp_b[0][n - 1]
    }
}

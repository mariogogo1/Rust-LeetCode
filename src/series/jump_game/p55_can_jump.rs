/**
55. 跳跃游戏

给你一个非负整数数组 nums ，你最初位于数组的 第一个下标 。数组中的每个元素代表你在该位置可以跳跃的最大长度。

判断你是否能够到达最后一个下标，如果可以，返回 true ；否则，返回 false 。

https://leetcode.cn/problems/jump-game/description/
*/

// 貪心 往前跳
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach = 0;
        let mut i: usize = 0;
        while i <= max_reach {
            if nums[i] as usize + i > max_reach {
                max_reach = nums[i] as usize + i;
            }
            if max_reach >= nums.len() - 1 {
                return true;
            }
            i += 1;
        }

        false
    }
}

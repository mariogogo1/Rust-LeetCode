/**
45. 跳跃游戏 II

给定一个长度为 n 的 0 索引整数数组 nums。初始位置为 nums[0]。

每个元素 nums[i] 表示从索引 i 向前跳转的最大长度。换句话说，如果你在 nums[i] 处，你可以跳转到任意 nums[i + j] 处:

0 <= j <= nums[i]
i + j < n
返回到达 nums[n - 1] 的最小跳跃次数。生成的测试用例可以到达 nums[n - 1]。

https://leetcode.cn/problems/jump-game-ii/description/
*/
// 類似55提 看更新了幾次
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        let mut end = 0;
        let mut max_position = 0;
        let mut steps = 0;

        for i in 0..length - 1 {
            max_position = max_position.max(i as i32 + nums[i]);
            if i as i32 == end {
                end = max_position;
                steps += 1;
            }
        }

        steps
    }
}

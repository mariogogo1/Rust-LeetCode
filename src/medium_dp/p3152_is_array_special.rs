/**
3152. 特殊数组 II

如果数组的每一对相邻元素都是两个奇偶性不同的数字，则该数组被认为是一个 特殊数组 。

周洋哥有一个整数数组 nums 和一个二维整数矩阵 queries，对于 queries[i] = [fromi, toi]，请你帮助周洋哥检查子数组 nums[fromi..toi] 是不是一个 特殊数组 。

返回布尔数组 answer，如果 nums[fromi..toi] 是特殊数组，则 answer[i] 为 true ，否则，answer[i] 为 false 。

https://leetcode.cn/problems/special-array-ii/description/
*/
impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();
        let mut dp_special_len: Vec<usize> = vec![0; n];
        for i in 1..nums.len() {
            if (nums[i] + nums[i - 1]) % 2 == 1 {
                dp_special_len[i] = dp_special_len[i - 1];
            } else {
                dp_special_len[i] = i;
            }
        }
        let mut ans: Vec<bool> = vec![true; queries.len()];
        for (i, &ref query) in queries.iter().enumerate() {
            if query[0] != query[1] && dp_special_len[query[1] as usize] > query[0] as usize {
                ans[i] = false;
            }
        }
        ans
    }
}

/**
135. 分发糖果

n 个孩子站成一排。给你一个整数数组 ratings 表示每个孩子的评分。

你需要按照以下要求，给这些孩子分发糖果：

每个孩子至少分配到 1 个糖果。
相邻两个孩子评分更高的孩子会获得更多的糖果。
请你给每个孩子分发糖果，计算并返回需要准备的 最少糖果数目 。

https://leetcode.cn/problems/candy/description/
*/
pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut ans = 1;
        let mut inc = 1;
        let mut dec = 0;
        let mut per = 1;

        for i in 1..ratings.len() {
            if ratings[i] >= ratings[i - 1] {
                if ratings[i] > ratings[i - 1] {
                    per += 1;
                } else {
                    per = 1;
                }
                dec = 0;
                inc = per;
                ans += inc;
            } else {
                dec += 1;
                if dec == inc {
                    dec += 1;
                }
                ans += dec;
                per = 1;
            }
        }

        ans
    }
}

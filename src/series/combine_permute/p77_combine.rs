/**
77. 组合
给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。

你可以按 任何顺序 返回答案。

https://leetcode.cn/problems/combinations/description/
*/
pub struct Solution;
impl Solution {
    fn dfs(
        candidates: &Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        one_ans: &mut Vec<i32>,
        count: i32,
        idx: usize,
    ) {
        if count == 0 {
            ans.push(one_ans.clone());

            return;
        }
        for i in idx..candidates.len() {
            one_ans.push(candidates[i]);
            Self::dfs(candidates, ans, one_ans, count - 1, i + 1);
            one_ans.pop();
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let candidates = (1..=n).collect();

        let mut ans = Vec::new();

        Self::dfs(&candidates, &mut ans, &mut Vec::new(), k, 0);

        ans
    }
}

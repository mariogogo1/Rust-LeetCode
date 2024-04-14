/**
40. 组合总和 II
给定一个候选人编号的集合 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。

candidates 中的每个数字在每个组合中只能使用 一次 。

注意：解集不能包含重复的组合。

https://leetcode.cn/problems/combination-sum-ii/description/
*/
pub struct Solution;
impl Solution {
    fn dfs(
        candidates: &Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        one_ans: &mut Vec<i32>,
        target: i32,
        idx: usize,
    ) {
        if target == 0 {
            ans.push(one_ans.clone());
            return;
        }
        for i in idx..candidates.len() {
            if candidates[i] > target {
                break;
            }
            if i > idx && candidates[i] == candidates[i - 1] {
                continue;
            }

            one_ans.push(candidates[i]);
            Self::dfs(candidates, ans, one_ans, target - candidates[i], i + 1);
            one_ans.pop();
        }
    }

    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        let mut ans = Vec::new();

        Self::dfs(&candidates, &mut ans, &mut Vec::new(), target, 0);

        ans
    }
}

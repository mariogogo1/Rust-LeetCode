/**
39. 组合总和

给你一个 无重复元素 的整数数组 candidates 和一个目标整数 target ，找出 candidates 中可以使数字和为目标数 target 的 所有 不同组合 ，并以列表形式返回。你可以按 任意顺序 返回这些组合。

candidates 中的 同一个 数字可以 无限制重复被选取 。如果至少一个数字的被选数量不同，则两种组合是不同的。

对于给定的输入，保证和为 target 的不同组合数少于 150 个。

https://leetcode.cn/problems/combination-sum/description/
*/
pub struct Solution;

impl Solution {
    fn dfs(
        ans: &mut Vec<Vec<i32>>,
        one_ans: &mut Vec<i32>,
        candidates: &Vec<i32>,
        target: i32,
        idx: usize,
    ) {
        if target == 0 {
            ans.push(one_ans.clone());
            return;
        }
        for i in idx..candidates.len() {
            let num = candidates[i];
            if target >= num {
                one_ans.push(num);
                Self::dfs(ans, one_ans, candidates, target - num, i);
                one_ans.pop();
            }
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut one_ans: Vec<i32> = Vec::new();

        Self::dfs(&mut ans, &mut one_ans, &candidates, target, 0);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
    }
    #[test]
    fn test_case() {}
}

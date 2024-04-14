/**
216. 组合总和 III
找出所有相加之和为 n 的 k 个数的组合，且满足下列条件：

只使用数字1到9
每个数字 最多使用一次
返回 所有可能的有效组合的列表 。该列表不能包含相同的组合两次，组合可以以任何顺序返回。

https://leetcode.cn/problems/combination-sum-iii/description/
*/
pub struct Solution;
impl Solution {
    fn dfs(
        candidates: &Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        one_ans: &mut Vec<i32>,
        target: i32,
        count: i32,
        idx: usize,
    ) {
        if count == 0 {
            if target == 0 {
                ans.push(one_ans.clone());
            }
            return;
        }
        for i in idx..candidates.len() {
            if candidates[i] > target {
                break;
            }
            one_ans.push(candidates[i]);
            Self::dfs(
                candidates,
                ans,
                one_ans,
                target - candidates[i],
                count - 1,
                i + 1,
            );
            one_ans.pop();
        }
    }

    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let candidates = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut ans = Vec::new();

        Self::dfs(&candidates, &mut ans, &mut Vec::new(), n, k, 0);

        ans
    }
}

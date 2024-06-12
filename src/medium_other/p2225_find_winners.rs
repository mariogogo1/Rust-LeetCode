/**
2225. 找出输掉零场或一场比赛的玩家

给你一个整数数组 matches 其中 matches[i] = [winneri, loseri] 表示在一场比赛中 winneri 击败了 loseri 。

返回一个长度为 2 的列表 answer ：

answer[0] 是所有 没有 输掉任何比赛的玩家列表。
answer[1] 是所有恰好输掉 一场 比赛的玩家列表。
两个列表中的值都应该按 递增 顺序返回。

注意：

只考虑那些参与 至少一场 比赛的玩家。
生成的测试用例保证 不存在 两场比赛结果 相同 。

https://leetcode.cn/problems/find-players-with-zero-or-one-losses/description/
*/
pub struct Solution;
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut count = vec![-1; 100001];
        let mut answer = vec![vec![], vec![]];
        let mut max_idx = 0;
        for the_match in matches {
            max_idx = max_idx.max(the_match[0]);
            max_idx = max_idx.max(the_match[1]);
            if count[the_match[0] as usize] < 0 {
                count[the_match[0] as usize] += 1;
            }

            if count[the_match[1] as usize] < 0 {
                count[the_match[1] as usize] += 1;
            }

            count[the_match[1] as usize] += 1;
        }
        for i in 1..=max_idx {
            match count[i as usize] {
                0 => answer[0].push(i),
                1 => answer[1].push(i),
                _ => continue,
            }
        }
        answer
    }
}

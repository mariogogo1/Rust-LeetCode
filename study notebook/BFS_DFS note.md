# BFS & DFS & TRACEBACK 模板筆記    
1.BFS
  pub fn BFS() -> i32 {
        let mut visited = vec![false; n];
        let mut start = vec![0];
        let target_num = target;
 
        let mut step = 0;

        while !start.is_empty() {
            let mut new_start = vec![];
            for &v in &start {
                if v == target_num {
                    return step;
                }
                if visited[v] {
                    continue;
                }
                visited[v] = true;
                new_start.extend(NEXT_POINTS);
            }
            start = new_start;
            step += 1;
        }
        step
    }

2.DFS
impl Solution {
    fn dfs(ans: &mut Vec<Vec<i32>>, one_ans: &mut Vec<i32>, target: i32) {}

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut one_ans: Vec<i32> = Vec::new();

        Self::dfs(&mut ans, &mut one_ans, target);
        ans
    }
}

//39. 组合总和
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
            let num = candidates[idx];
            one_ans.push(num);
            Self::dfs(ans, one_ans, &candidates, target - num, i + 1);
            one_ans.pop();
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut one_ans: Vec<i32> = Vec::new();

        Self::dfs(&mut ans, &mut one_ans, &candidates, target, 0);
        ans
    }
}



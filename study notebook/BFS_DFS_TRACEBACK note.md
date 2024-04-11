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


3.TRACEBACK




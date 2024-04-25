/**
1931. 用三种不同颜色为网格涂色

给你两个整数 m 和 n 。构造一个 m x n 的网格，其中每个单元格最开始是白色。请你用 红、绿、蓝 三种颜色为每个单元格涂色。所有单元格都需要被涂色。

涂色方案需要满足：不存在相邻两个单元格颜色相同的情况 。返回网格涂色的方法数。因为答案可能非常大， 返回 对 109 + 7 取余 的结果。


https://leetcode.cn/problems/painting-a-grid-with-three-different-colors/description/
*/
pub struct Solution;
/// 1931 跟 1411相同，1411略為簡單一些
impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;

    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;
        let mut state_vec: Vec<Vec<i32>> = vec![vec![]];
        let mut new_state_vec: Vec<Vec<i32>> = vec![];

        for _ in 0..m {
            new_state_vec.clear();
            for state in state_vec {
                if let Some(&last_color) = state.last() {
                    for color in 0..3 {
                        let mut s = state.clone();
                        if last_color != color {
                            s.push(color);
                            new_state_vec.push(s);
                        }
                    }
                } else {
                    for color in 0..3 {
                        let mut s = state.clone();
                        s.push(color);
                        new_state_vec.push(s);
                    }
                }
            }
            state_vec = new_state_vec.clone();
        }

        if n == 1 {
            return state_vec.len() as i32;
        }

        // 找出轉移矩陣
        let mut transpose: Vec<Vec<usize>> = vec![Vec::new(); state_vec.len()];
        for i in 0..state_vec.len() {
            'loop2: for j in (i + 1)..state_vec.len() {
                for k in 0..m {
                    if state_vec[i][k] == state_vec[j][k] {
                        continue 'loop2;
                    }
                }
                transpose[i].push(j);
                transpose[j].push(i);
            }
        }

        let mut dp: Vec<Vec<i64>> = vec![vec![1; state_vec.len()]; n];

        for i in 1..n {
            for j in 0..state_vec.len() {
                dp[i][j] = 0;
                for idx in &transpose[j] {
                    dp[i][j] += dp[i - 1][*idx];
                }
                dp[i][j] %= Self::VAL_MOD;
            }
        }

        let mut ans = 0;

        for j in 0..state_vec.len() {
            ans += dp[n - 1][j];
            ans %= Self::VAL_MOD;
        }

        ans as i32
    }
}

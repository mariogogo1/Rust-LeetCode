/**
51. N 皇后

按照国际象棋的规则，皇后可以攻击与之处在同一行或同一列或同一斜线上的棋子。

n 皇后问题 研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。

给你一个整数 n ，返回所有不同的 n 皇后问题 的解决方案。

每一种解法包含一个不同的 n 皇后问题 的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。
Hint:
1 <= n <= 9

https://leetcode.cn/problems/n-queens/description/
*/
pub struct Solution;

struct QueenBool {
    col: Vec<bool>,
    // 直觀紀錄左下到右上的方向線上有無皇后
    lb_diagonal: Vec<bool>,
    // 直觀紀錄右下到左上的方向線上有無皇后
    rb_diagonal: Vec<bool>,
}

fn traceback(
    step: usize,
    one_ans: &mut Vec<Vec<char>>,
    q: &mut QueenBool,
    ans: &mut Vec<Vec<String>>,
) {
    let n_usize = one_ans.len();
    if step == n_usize {
        ans.push(make_ans(one_ans));
        return;
    }
    for i in 0..n_usize {
        if !q.col[i] && !q.lb_diagonal[i + step] && !q.rb_diagonal[n_usize - 1 - step + i] {
            q.col[i] = true;
            q.lb_diagonal[i + step] = true;
            q.rb_diagonal[n_usize - 1 - step + i] = true;
            one_ans[step][i] = 'Q';

            traceback(step + 1, one_ans, q, ans);

            q.col[i] = false;
            q.lb_diagonal[i + step] = false;
            q.rb_diagonal[n_usize - 1 - step + i] = false;
            one_ans[step][i] = '.';
        }
    }
}
fn make_ans(one_ans: &Vec<Vec<char>>) -> Vec<String> {
    let mut ans: Vec<String> = Vec::with_capacity(one_ans.len());
    for row in one_ans.iter() {
        ans.push(row.iter().collect());
    }
    ans
}
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n_usize = n as usize;
        let mut ans: Vec<Vec<String>> = vec![];
        let mut one_ans: Vec<Vec<char>> = vec![vec!['.'; n_usize]; n_usize];
        let mut q: QueenBool = QueenBool {
            col: vec![false; n_usize],
            lb_diagonal: vec![false; 2 * n_usize - 1],
            rb_diagonal: vec![false; 2 * n_usize - 1],
        };

        traceback(0, &mut one_ans, &mut q, &mut ans);
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![
                    ".Q..".to_string(),
                    "...Q".to_string(),
                    "Q...".to_string(),
                    "..Q.".to_string()
                ],
                vec![
                    "..Q.".to_string(),
                    "Q...".to_string(),
                    "...Q".to_string(),
                    ".Q..".to_string()
                ]
            ]
        );
        // assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q".to_string()]]);
    }
    #[test]
    fn test_case() {}
}

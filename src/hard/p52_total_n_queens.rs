/**
52. N 皇后 II

n 皇后问题 研究的是如何将 n 个皇后放置在 n × n 的棋盘上，并且使皇后彼此之间不能相互攻击。

给你一个整数 n ，返回 n 皇后问题 不同的解决方案的数量。
Hint:
1 <= n <= 9

https://leetcode.cn/problems/n-queens-ii/description/
*/
pub struct Solution;

struct QueenBool {
    col: Vec<bool>,
    // 直觀紀錄左下到右上的方向線上有無皇后
    lb_diagonal: Vec<bool>,
    // 直觀紀錄右下到左上的方向線上有無皇后
    rb_diagonal: Vec<bool>,
}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n_usize = n as usize;
        let mut ans: i32 = 0;
        let mut q: QueenBool = QueenBool {
            col: vec![false; n_usize],
            lb_diagonal: vec![false; 2 * n_usize - 1],
            rb_diagonal: vec![false; 2 * n_usize - 1],
        };

        Solution::traceback(0, &mut q, &mut ans);
        return ans;
    }

    fn traceback(step: usize, q: &mut QueenBool, ans: &mut i32) {
        let n_usize = q.col.len();
        if step == n_usize {
            *ans += 1;
            return;
        }
        for i in 0..n_usize {
            if !q.col[i] && !q.lb_diagonal[i + step] && !q.rb_diagonal[n_usize - 1 - step + i] {
                q.col[i] = true;
                q.lb_diagonal[i + step] = true;
                q.rb_diagonal[n_usize - 1 - step + i] = true;

                Self::traceback(step + 1, q, ans);

                q.col[i] = false;
                q.lb_diagonal[i + step] = false;
                q.rb_diagonal[n_usize - 1 - step + i] = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::total_n_queens(4), 2);
    }
    #[test]
    fn test_case() {}
}

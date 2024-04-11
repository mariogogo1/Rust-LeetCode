/**
面试题 08.06. 汉诺塔问题

在经典汉诺塔问题中，有 3 根柱子及 N 个不同大小的穿孔圆盘，盘子可以滑入任意一根柱子。一开始，所有盘子自上而下按升序依次套在第一根柱子上(即每一个盘子只能放在更大的盘子上面)。移动圆盘时受到以下限制:
(1) 每次只能移动一个盘子;
(2) 盘子只能从柱子顶端滑出移到下一根柱子;
(3) 盘子只能叠在比它大的盘子上。

请编写程序，用栈将所有盘子从第一根柱子移到最后一根柱子。

你需要原地修改栈。
https://leetcode.cn/problems/hanota-lcci/description/
*/
pub struct Solution;
impl Solution {
    pub fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        let n = a.len();
        Self::move_dice(n, a, b, c);
    }
    // 把n個盤子從a移到c透過b
    fn move_dice(n: usize, a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        if n == 1 {
            if let Some(x) = a.pop() {
                c.push(x);
            }
            return;
        }

        Self::move_dice(n - 1, a, c, b); // 把n-1個盤子從a移到b透過c
        Self::move_dice(1, a, b, c); //   把1個盤子從a移到c透過b
        Self::move_dice(n - 1, b, a, c); //把n-1個盤子從b移到c透過a
    }
}

/**
面试题 08.13. 堆箱子

堆箱子。给你一堆n个箱子，箱子宽 wi、深 di、高 hi。箱子不能翻转，将箱子堆起来时，下面箱子的宽度、高度和深度必须大于上面的箱子。实现一种方法，搭出最高的一堆箱子。箱堆的高度为每个箱子高度的总和。

输入使用数组[wi, di, hi]表示每个箱子。

https://leetcode.cn/problems/pile-box-lcci/description/
*/

pub struct Solution;
/// 前置練習 300題 354題
/// 使用動態規劃
impl Solution {
    pub fn pile_box(mut boxes: Vec<Vec<i32>>) -> i32 {
        boxes.sort_unstable_by(|a, b| a[2].cmp(&b[2]));
        let mut dp = vec![0; boxes.len()];
        let mut ans = 0;
        for i in 0..boxes.len() {
            let mut count = 0;
            for j in 0..i {
                if boxes[i][0] > boxes[j][0]
                    && boxes[i][1] > boxes[j][1]
                    && boxes[i][2] > boxes[j][2]
                {
                    count = count.max(dp[j]);
                }
            }
            dp[i] = count + boxes[i][2];
            ans = ans.max(dp[i]);
        }
        ans
    }
}

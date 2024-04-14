/**
1561. 你可以获得的最大硬币数目

有 3n 堆数目不一的硬币，你和你的朋友们打算按以下方式分硬币：

每一轮中，你将会选出 任意 3 堆硬币（不一定连续）。
Alice 将会取走硬币数量最多的那一堆。
你将会取走硬币数量第二多的那一堆。
Bob 将会取走最后一堆。
重复这个过程，直到没有更多硬币。
给你一个整数数组 piles ，其中 piles[i] 是第 i 堆中硬币的数目。

返回你可以获得的最大硬币数目。

https://leetcode.cn/problems/maximum-number-of-coins-you-can-get/description/
*/
pub struct Solution;
//排序後拿走第2大.第4大...的數字總和
impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let x: usize = n / 3;
        piles.sort_unstable();
        let mut ans = 0;
        for i in 0..x {
            ans += piles[n - 2 - 2 * i];
        }
        ans
    }
}

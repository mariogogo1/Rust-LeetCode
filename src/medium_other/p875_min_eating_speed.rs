/**
875. 爱吃香蕉的珂珂

珂珂喜欢吃香蕉。这里有 n 堆香蕉，第 i 堆中有 piles[i] 根香蕉。警卫已经离开了，将在 h 小时后回来。

珂珂可以决定她吃香蕉的速度 k （单位：根/小时）。每个小时，她将会选择一堆香蕉，从中吃掉 k 根。如果这堆香蕉少于 k 根，她将吃掉这堆的所有香蕉，然后这一小时内不会再吃更多的香蕉。

珂珂喜欢慢慢吃，但仍然想在警卫回来前吃掉所有的香蕉。

返回她可以在 h 小时内吃掉所有香蕉的最小速度 k（k 为整数）。

https://leetcode.cn/problems/koko-eating-bananas/description/
*/
pub struct Solution;
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let h = h as i64;
        let mut l = 0 as i64;
        let mut r = i32::MAX as i64;

        while l < r {
            let mut m = l + (r - l) / 2;
            if m == 0 {
                return 1;
            }
            let mut time = 0;
            for i in 0..piles.len() {
                time += (piles[i] as i64 - 1) / m + 1;
            }
            if time <= h {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l as i32
    }
}

/**
605. 种花问题

假设有一个很长的花坛，一部分地块种植了花，另一部分却没有。可是，花不能种植在相邻的地块上，它们会争夺水源，两者都会死去。

给你一个整数数组 flowerbed 表示花坛，由若干 0 和 1 组成，其中 0 表示没种植花，1 表示种植了花。另有一个数 n ，能否在不打破种植规则的情况下种入 n 朵花？能则返回 true ，不能则返回 false 。

https://leetcode.cn/problems/can-place-flowers/description/
*/

pub struct Solution;
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut ans = 0;
        let mut count = 1;
        for flower in flowerbed {
            if flower == 1 {
                ans += (count - 1) / 2;
                count = 0;
            } else {
                count += 1;
            }
        }
        ans += count / 2;

        return ans >= n;
    }
}

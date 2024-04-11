/**
354. 俄罗斯套娃信封问题

给你一个二维整数数组 envelopes ，其中 envelopes[i] = [wi, hi] ，表示第 i 个信封的宽度和高度。

当另一个信封的宽度和高度都比这个信封大的时候，这个信封就可以放进另一个信封里，如同俄罗斯套娃一样。

请计算 最多能有多少个 信封能组成一组“俄罗斯套娃”信封（即可以把一个信封放到另一个信封里面）。

注意：不允许旋转信封。
https://leetcode.cn/problems/russian-doll-envelopes/description/
*/
/// 這題很關鍵，幾乎是遞增子序列的基礎題目，俄羅斯套娃信封這種經典題目就是從這裡延伸的
pub struct Solution;

/// 前置練習 300題
/// 將信封依照長度做遞增排序，同高的信封按照寬度做遞減排序
/// 維護信封的遞增序列
impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut tails = Vec::new();
        for env in envelopes {
            let pos = tails.binary_search(&env[1]).unwrap_or_else(|x| x);
            if pos == tails.len() {
                tails.push(env[1]);
            } else {
                tails[pos] = env[1];
            }
        }
        tails.len() as i32
    }
}

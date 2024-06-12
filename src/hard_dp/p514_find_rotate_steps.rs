/**
514. 自由之路

电子游戏“辐射4”中，任务 “通向自由” 要求玩家到达名为 “Freedom Trail Ring” 的金属表盘，并使用表盘拼写特定关键词才能开门。

给定一个字符串 ring ，表示刻在外环上的编码；给定另一个字符串 key ，表示需要拼写的关键词。您需要算出能够拼写关键词中所有字符的最少步数。

最初，ring 的第一个字符与 12:00 方向对齐。您需要顺时针或逆时针旋转 ring 以使 key 的一个字符在 12:00 方向对齐，然后按下中心按钮，以此逐个拼写完 key 中的所有字符。

旋转 ring 拼出 key 字符 key[i] 的阶段中：

您可以将 ring 顺时针或逆时针旋转 一个位置 ，计为1步。旋转的最终目的是将字符串 ring 的一个字符与 12:00 方向对齐，并且这个字符必须等于字符 key[i] 。
如果字符 key[i] 已经对齐到12:00方向，您需要按下中心按钮进行拼写，这也将算作 1 步。按完之后，您可以开始拼写 key 的下一个字符（下一阶段）, 直至完成所有拼写。

https://leetcode.cn/problems/freedom-trail/description/
*/

pub struct Solution;
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring: Vec<char> = ring.chars().collect();
        let key: Vec<char> = key.chars().collect();
        let mut dp = vec![vec![0; key.len()]; ring.len()];
        let mut start_pos = Vec::new();

        for (k, &r) in ring.iter().enumerate() {
            if r == key[0] {
                dp[k][0] = 1 + k.min(ring.len() - k) as i32;
                start_pos.push(k);
            }
        }

        let mut end_pos;
        let mut min_v;
        for i in 1..key.len() {
            end_pos = Vec::new();
            for j in 0..ring.len() {
                if ring[j] == key[i] {
                    min_v = i32::MAX;
                    for &v in &start_pos {
                        let dist1 = (j as i32 - v as i32).abs();
                        let dist2 = (ring.len() as i32 - dist1).abs();
                        min_v = min_v.min(1 + dp[v][i - 1] + dist1.min(dist2));
                    }
                    end_pos.push(j);
                    dp[j][i] = min_v;
                }
            }
            start_pos = end_pos.clone();
        }

        min_v = i32::MAX;
        for j in 0..ring.len() {
            if dp[j][key.len() - 1] > 0 {
                min_v = min_v.min(dp[j][key.len() - 1]);
            }
        }

        min_v
    }
}

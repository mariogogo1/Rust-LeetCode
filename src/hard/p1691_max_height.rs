/**
1691. 堆叠长方体的最大高度

给你 n 个长方体 cuboids ，其中第 i 个长方体的长宽高表示为 cuboids[i] = [widthi, lengthi, heighti]（下标从 0 开始）。请你从 cuboids 选出一个 子集 ，并将它们堆叠起来。

如果 widthi <= widthj 且 lengthi <= lengthj 且 heighti <= heightj ，你就可以将长方体 i 堆叠在长方体 j 上。你可以通过旋转把长方体的长宽高重新排列，以将它放在另一个长方体上。

返回 堆叠长方体 cuboids 可以得到的 最大高度 。

https://leetcode.cn/problems/maximum-height-by-stacking-cuboids/description/
*/

pub struct Solution;
/*
/// 把所有長方體按照 width<= length<= height 做旋轉
/// 再將所有長方體依照height的大小作升序排序
/// 之後的問題就同面试题 08.13. 堆箱子 一樣的作法
///
/// 說明：
/// 以最簡單的兩個長方體 i 跟 j 來觀察
/// 1.要使加總高度最高，長方體按照 width<= length<= height 做旋轉，是很直觀的想法，如果兩個可以疊上去，這必然是最佳做法
/// 2.如果這兩個無法疊上去，則其他的旋轉方式都無法相疊
///  方便說明 假設height[i] <= height[j],i準備要在j上面，
///  如果無法相疊會有三種情況
///  (1). width[i] <  width[j] 且 length[i] > length[j]
///      j旋轉為 [ width[j],height[j], length[j] ]，接觸面j可以比i大 可是length[j]成為高的時候 ->  length[j]< length[i] <= height[i] ，不符題意->無法相疊。
///                |  接觸面面積 |
///
///       考慮旋轉i 使得兩個可以相疊,因為 width[i] <  width[j] <= length[j] < length[i]
///       以 width[i] 跟 height[i]當接觸面，會發生新的高 length[i] > length[j] 或是 length[i] > width[j] 的問題，不符題意->無法相疊。
///       以 length[i] 跟 height[i]當接觸面，j沒有可以當接觸面的部分，不符題意->無法相疊。
///
///  (2). width[i] >  width[j] 且 length[i] < length[j]
///      類似(1)
///
///  (3).width[i] >  width[j] 且 length[i] > length[j]
///      無法旋轉使得接觸面滿足 width[i] <=  width[j] 且 length[i] <=  length[j]
*/

impl Solution {
    pub fn max_height(mut cuboids: Vec<Vec<i32>>) -> i32 {
        for cuboid in cuboids.iter_mut() {
            if cuboid[0] > cuboid[1] {
                cuboid.swap(0, 1);
            }
            if cuboid[1] > cuboid[2] {
                cuboid.swap(1, 2);
            }
            if cuboid[0] > cuboid[1] {
                cuboid.swap(0, 1);
            }
        }

        cuboids.sort_unstable_by(|a, b| {
            if a[2] != b[2] {
                a[2].cmp(&b[2])
            } else if a[1] != b[1] {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut dp = vec![0; cuboids.len()];
        let mut ans = 0;
        for i in 0..cuboids.len() {
            let mut count = 0;
            for j in 0..i {
                if cuboids[i][0] >= cuboids[j][0]
                    && cuboids[i][1] >= cuboids[j][1]
                    && cuboids[i][2] >= cuboids[j][2]
                {
                    count = count.max(dp[j]);
                }
            }
            dp[i] = count + cuboids[i][2];
            ans = ans.max(dp[i]);
        }
        ans
    }
}

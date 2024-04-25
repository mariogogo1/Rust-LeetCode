/**
2136. 全部开花的最早一天

你有 n 枚花的种子。每枚种子必须先种下，才能开始生长、开花。播种需要时间，种子的生长也是如此。给你两个下标从 0 开始的整数数组 plantTime 和 growTime ，每个数组的长度都是 n ：

plantTime[i] 是 播种 第 i 枚种子所需的 完整天数 。每天，你只能为播种某一枚种子而劳作。无须 连续几天都在种同一枚种子，但是种子播种必须在你工作的天数达到 plantTime[i] 之后才算完成。
growTime[i] 是第 i 枚种子完全种下后生长所需的 完整天数 。在它生长的最后一天 之后 ，将会开花并且永远 绽放 。
从第 0 开始，你可以按 任意 顺序播种种子。

返回所有种子都开花的 最早 一天是第几天。
https://leetcode.cn/problems/earliest-possible-day-of-full-bloom/description/
*/
pub struct Solution;

// 考慮生長期grow_time的時間由長排到短 -(1)
// 假設在i-1種花種下後是最短時間的情況下種下第i朵花
// 不存在更加的種花順序的使得都開花的時間更短
//
// 證明： 假設存在一個更好的新種花順序
// 新的種花順序交換後 = plant_time_1+...+plant_time_i + grow_time_x <  plant_time_1+...+plant_time_i + grow_time_i
// grow_time_x < grow_time_i -跟(1)矛盾
// 上面證明省略了一種情況，可以留給大家自行想想
//
impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut pair: Vec<(&i32, &i32)> = grow_time.iter().zip(plant_time.iter()).collect();
        pair.sort_unstable_by(|i, j| j.0.cmp(&i.0));

        let mut ans = 0;
        let mut sum = 0;

        for i in 0..grow_time.len() {
            sum += pair[i].1;
            ans = ans.max(sum + pair[i].0);
        }

        ans
    }
}

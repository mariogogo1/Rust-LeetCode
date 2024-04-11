/**
2751. 机器人碰撞

现有 n 个机器人，编号从 1 开始，每个机器人包含在路线上的位置、健康度和移动方向。

给你下标从 0 开始的两个整数数组 positions、healths 和一个字符串 directions（directions[i] 为 'L' 表示 向左 或 'R' 表示 向右）。 positions 中的所有整数 互不相同 。

所有机器人以 相同速度 同时 沿给定方向在路线上移动。如果两个机器人移动到相同位置，则会发生 碰撞 。

如果两个机器人发生碰撞，则将 健康度较低 的机器人从路线中 移除 ，并且另一个机器人的健康度 减少 1 。幸存下来的机器人将会继续沿着与之前 相同 的方向前进。如果两个机器人的健康度相同，则将二者都从路线中移除。

请你确定全部碰撞后幸存下的所有机器人的 健康度 ，并按照原来机器人编号的顺序排列。即机器人 1 （如果幸存）的最终健康度，机器人 2 （如果幸存）的最终健康度等。 如果不存在幸存的机器人，则返回空数组。

在不再发生任何碰撞后，请你以数组形式，返回所有剩余机器人的健康度（按机器人输入中的编号顺序）。

注意：位置  positions 可能是乱序的。

https://leetcode.cn/problems/robot-collisions/description/
*/
pub struct Solution;
/// 排序 + 用棧模擬機器人碰撞
impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let mut data_vec: Vec<(i32, i32, bool, usize)> = Vec::new();
        for (i, c) in directions.chars().enumerate() {
            if c == 'L' {
                data_vec.push((positions[i], healths[i], true, i));
            } else {
                data_vec.push((positions[i], healths[i], false, i));
            }
        }
        data_vec.sort_by_key(|&(pos, _, _, _)| pos);
        let mut ans = Vec::new();
        let mut stack = Vec::new();
        for i in 0..data_vec.len() {
            if !data_vec[i].2 {
                stack.push(i);
            } else {
                while let Some(&idx) = stack.last() {
                    if data_vec[i].1 > data_vec[idx].1 {
                        data_vec[i].1 -= 1;
                        data_vec[idx].1 = 0;
                        stack.pop();
                    } else if data_vec[i].1 == data_vec[idx].1 {
                        data_vec[i].1 = 0;
                        data_vec[idx].1 = 0;
                        stack.pop();
                    } else {
                        data_vec[idx].1 -= 1;
                        data_vec[i].1 = 0;
                        break;
                    }
                }
            }
        }
        data_vec.sort_by_key(|&(_, _, _, idx)| idx);

        for i in 0..data_vec.len() {
            if data_vec[i].1 > 0 {
                ans.push(data_vec[i].1);
            }
        }
        ans
    }
}

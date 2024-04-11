/**
1776. 车队 II

在一条单车道上有 n 辆车，它们朝着同样的方向行驶。给你一个长度为 n 的数组 cars ，其中 cars[i] = [positioni, speedi] ，它表示：

positioni 是第 i 辆车和道路起点之间的距离（单位：米）。题目保证 positioni < positioni+1 。
speedi 是第 i 辆车的初始速度（单位：米/秒）。
简单起见，所有车子可以视为在数轴上移动的点。当两辆车占据同一个位置时，我们称它们相遇了。一旦两辆车相遇，它们会合并成一个车队，这个车队里的车有着同样的位置和相同的速度，速度为这个车队里 最慢 一辆车的速度。

请你返回一个数组 answer ，其中 answer[i] 是第 i 辆车与下一辆车相遇的时间（单位：秒），如果这辆车不会与下一辆车相遇，则 answer[i] 为 -1 。答案精度误差需在 10-5 以内。

https://leetcode.cn/problems/car-fleet-ii/description/
*/
struct Solution {}
impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        let mut stack: Vec<usize> = Vec::new();
        let mut sec_stack: Vec<f64> = Vec::new(); // 記錄秒數
        let mut ans: Vec<f64> = vec![-1.0; cars.len()];
        let mut leader_car_index: usize = cars.len() - 1;
        for i in (0..cars.len()).rev() {
            // 如果車速更小 就變成新的領導車隊 他無法追上下標更後面的車隊
            if cars[i][1] <= cars[leader_car_index][1] {
                stack.clear();
                sec_stack.clear();
                leader_car_index = i;
                stack.push(i);
                sec_stack.push(f64::MAX);
                continue;
            }
            //println!("{:?}", stack);
            let mut chase_second: f64 = -1.0;
            while let Some(&j) = stack.last() {
                let Some(&sec) = sec_stack.last() else {
                    todo!()
                };
                chase_second =
                    ((cars[j][0] - cars[i][0]) as f64) / ((cars[i][1] - cars[j][1]) as f64);

                if chase_second > sec || chase_second < 0.0 {
                    stack.pop();
                    sec_stack.pop();
                } else {
                    break;
                }
            }
            ans[i] = chase_second;
            stack.push(i);
            sec_stack.push(chase_second);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::get_collision_times(vec![vec![0, 3], vec![10, 2], vec![12, 1]]),
            vec![6.00000, 2.00000, -1.00000]
        );
        assert_eq!(
            Solution::get_collision_times(vec![vec![1, 2], vec![2, 1], vec![4, 3], vec![7, 2]]),
            vec![1.00000, -1.00000, 3.00000, -1.00000]
        );
        assert_eq!(
            Solution::get_collision_times(vec![vec![3, 4], vec![5, 4], vec![6, 3], vec![9, 1]]),
            vec![2.00000, 1.00000, 1.50000, -1.00000]
        );
    }
    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::get_collision_times(vec![
                vec![1, 4],
                vec![4, 5],
                vec![7, 1],
                vec![13, 4],
                vec![14, 3],
                vec![15, 2],
                vec![16, 5],
                vec![19, 1]
            ]),
            vec![2.0, 0.75, -1.0, 1.0, 1.0, 4.0, 0.75, -1.0]
        );
    }
    #[test]
    fn test_case_2() {}
}

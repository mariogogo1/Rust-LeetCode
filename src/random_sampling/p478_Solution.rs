/**
478. 在圆内随机生成点

给定圆的半径和圆心的位置，实现函数 randPoint ，在圆中产生均匀随机点。

实现 Solution 类:

Solution(double radius, double x_center, double y_center) 用圆的半径 radius 和圆心的位置 (x_center, y_center) 初始化对象
randPoint() 返回圆内的一个随机点。圆周上的一点被认为在圆内。答案作为数组返回 [x, y] 。

提示：

0 < radius <= 108
-107 <= x_center, y_center <= 107
randPoint 最多被调用 3 * 104 次

https://leetcode.cn/problems/generate-random-point-in-a-circle/description/
*/
use rand::{prelude::*, thread_rng};

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        return Solution {
            radius,
            x_center,
            y_center,
        };
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = thread_rng();
        let mut x: f64;
        let mut y: f64;
        loop {
            x = rng.gen_range(-1.0..=1.0);
            y = rng.gen_range(-1.0..=1.0);
            if x * x + y * y <= 1.0 {
                break;
            }
        }
        return vec![
            self.x_center + x * self.radius,
            self.y_center + y * self.radius,
        ];
    }
}

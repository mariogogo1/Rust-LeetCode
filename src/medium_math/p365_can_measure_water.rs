/**
365. 水壶问题

有两个水壶，容量分别为 x 和 y 升。水的供应是无限的。确定是否有可能使用这两个壶准确得到 target 升。

你可以：

装满任意一个水壶
清空任意一个水壶
将水从一个水壶倒入另一个水壶，直到接水壶已满，或倒水壶已空。

https://leetcode.cn/problems/water-and-jug-problem/description/
*/

pub struct Solution;

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
        let factor = Self::gcd(x, y);
        return target % factor == 0 && target <= x + y;
    }

    fn gcd<T: std::cmp::PartialEq + std::ops::Rem<Output = T> + Default + Copy>(
        mut a: T,
        mut b: T,
    ) -> T {
        while b != T::default() {
            (a, b) = (b, a % b)
        }
        a
    }
}

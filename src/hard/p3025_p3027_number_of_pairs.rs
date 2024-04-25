/**
3025. 人员站位的方案数 I
3027. 人员站位的方案数 II

给你一个  n x 2 的二维数组 points ，它表示二维平面上的一些点坐标，其中 points[i] = [xi, yi] 。

我们定义 x 轴的正方向为 右 （x 轴递增的方向），x 轴的负方向为 左 （x 轴递减的方向）。类似的，我们定义 y 轴的正方向为 上 （y 轴递增的方向），y 轴的负方向为 下 （y 轴递减的方向）。

你需要安排这 n 个人的站位，这 n 个人中包括 Alice 和 Bob 。你需要确保每个点处 恰好 有 一个 人。同时，Alice 想跟 Bob 单独玩耍，所以 Alice 会以 Alice 的坐标为 左上角 ，Bob 的坐标为 右下角 建立一个矩形的围栏（注意，围栏可能 不 包含任何区域，也就是说围栏可能是一条线段）。如果围栏的 内部 或者 边缘 上有任何其他人，Alice 都会难过。

请你在确保 Alice 不会 难过的前提下，返回 Alice 和 Bob 可以选择的 点对 数目。

提示：

2 <= n <= 1000
points[i].length == 2
-109 <= points[i][0], points[i][1] <= 109
points[i] 点对两两不同。
https://leetcode.cn/problems/find-the-number-of-ways-to-place-people-ii/description/
*/

pub struct Solution;

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|a, b| {
            if b[1] != a[1] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut ans = 0;
        for i in 0..(points.len() - 1) {
            let mut cur_x = i32::MAX;
            for j in i + 1..points.len() {
                if cur_x > points[j][0] && points[j][0] >= points[i][0] {
                    cur_x = points[j][0];
                    ans += 1;
                }
            }
        }

        ans
    }
}

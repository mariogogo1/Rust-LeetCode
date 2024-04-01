/**
497. 非重叠矩形中的随机点

给定一个由非重叠的轴对齐矩形的数组 rects ，其中 rects[i] = [ai, bi, xi, yi] 表示 (ai, bi) 是第 i 个矩形的左下角点，(xi, yi) 是第 i 个矩形的右上角点。设计一个算法来随机挑选一个被某一矩形覆盖的整数点。矩形周长上的点也算做是被矩形覆盖。所有满足要求的点必须等概率被返回。

在给定的矩形覆盖的空间内的任何整数点都有可能被返回。

请注意 ，整数点是具有整数坐标的点。

实现 Solution 类:

Solution(int[][] rects) 用给定的矩形数组 rects 初始化对象。
int[] pick() 返回一个随机的整数点 [u, v] 在给定的矩形所覆盖的空间内。

提示：

1 <= rects.length <= 100
rects[i].length == 4
-109 <= ai < xi <= 109
-109 <= bi < yi <= 109
xi - ai <= 2000
yi - bi <= 2000
所有的矩形不重叠。
pick 最多被调用 104 次。

https://leetcode.cn/problems/random-point-in-non-overlapping-rectangles/description/
*/
use rand::Rng;
struct Solution {
    rng: rand::rngs::ThreadRng,
    weight: Vec<i32>,
    rects: Vec<Vec<i32>>,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let n = rects.len();
        let mut solution = Solution {
            rng: rand::thread_rng(),
            rects,
            weight: vec![0; n + 1],
        };
        let mut sum = 0;
        for (i, rect) in solution.rects.iter().enumerate() {
            sum += (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1);
            solution.weight[i + 1] = sum;
        }
        return solution;
    }

    fn pick(&mut self) -> Vec<i32> {
        let n = self.weight.len();
        let mut prob: i32 = self.rng.gen_range(0..self.weight[n - 1]);
        let mut l: usize = 0;
        let mut r: usize = n - 1;
        while l < r {
            let m = (l + r) / 2;
            if prob >= self.weight[m + 1] {
                l = m + 1;
            } else if prob < self.weight[m] {
                r = m;
            } else {
                l = m;
                break;
            }
        }
        vec![
            self.rng.gen_range(self.rects[l][0]..=self.rects[l][2]),
            self.rng.gen_range(self.rects[l][1]..=self.rects[l][3]),
        ]
    }
}

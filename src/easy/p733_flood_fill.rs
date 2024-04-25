/**
733. 图像渲染

有一幅以 m x n 的二维整数数组表示的图画 image ，其中 image[i][j] 表示该图画的像素值大小。

你也被给予三个整数 sr ,  sc 和 newColor 。你应该从像素 image[sr][sc] 开始对图像进行 上色填充 。

为了完成 上色工作 ，从初始像素开始，记录初始坐标的 上下左右四个方向上 像素值与初始坐标相同的相连像素点，接着再记录这四个方向上符合条件的像素点与他们对应 四个方向上 像素值与初始坐标相同的相连像素点，……，重复该过程。将所有有记录的像素点的颜色值改为 newColor 。

最后返回 经过上色渲染后的图像 。

https://leetcode.cn/problems/flood-fill/description/
*/

pub struct Solution;

struct Directions {
    steps: Vec<(i32, i32)>,
}

impl Directions {
    pub fn new() -> Self {
        return Directions {
            steps: vec![(1, 0), (0, 1), (-1, 0), (0, -1)],
        };
    }
}

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let dirs = Directions::new();
        let n = image.len();
        let m = image[0].len();
        let target = image[sr as usize][sc as usize];

        if target != color {
            Self::dfs(&mut image, sr as usize, sc as usize, &dirs, target, color);
        }

        image
    }
    fn dfs(
        image: &mut Vec<Vec<i32>>,
        x: usize,
        y: usize,
        dirs: &Directions,
        target: i32,
        change: i32,
    ) {
        if image[x][y] == target {
            image[x][y] = change;
            for dir in &dirs.steps {
                let x = x as i32 + dir.0;
                let y = y as i32 + dir.1;

                if x >= 0 && x < image.len() as i32 && y >= 0 && y < image[0].len() as i32 {
                    Self::dfs(image, x as usize, y as usize, dirs, target, change);
                }
            }
        } else {
            return;
        }
    }
}

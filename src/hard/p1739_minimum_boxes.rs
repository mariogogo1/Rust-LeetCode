/**

1739. 放置盒子

有一个立方体房间，其长度、宽度和高度都等于 n 个单位。请你在房间里放置 n 个盒子，每个盒子都是一个单位边长的立方体。放置规则如下：

你可以把盒子放在地板上的任何地方。
如果盒子 x 需要放置在盒子 y 的顶部，那么盒子 y 竖直的四个侧面都 必须 与另一个盒子或墙相邻。
给你一个整数 n ，返回接触地面的盒子的 最少 可能数量。

提示：

1 <= n <= 109

https://leetcode.cn/problems/building-boxes/description/
*/
pub struct Solution;

///  題目已經展示最有效率的擺法：
///  
///  最底邊一定是 (1+2+3+4+...+m)，上一層是(1+2+3+4+...+(m-1))， 直到最上面為1
///  如果底邊長設為M
///  則能夠擺放的最多磚塊數量，為sum(x*(x+1)/2)，x=1~M
///  計算結果為 (M*(M+1)*(M+2)/6)
///  可用二分法找出底邊介於M~M+1 之間
///  
///  底面的接觸數量介於M*(M+1)/2 ~ (M+1)*(M+2)/2
///  再用二分法或是單純計算找出結果  
///
impl Solution {
    pub fn minimum_boxes(n: i32) -> i32 {
        let n_i64: i64 = n as i64;
        let mut r: i64 = 2000000;
        let mut l: i64 = 0;
        while l < r {
            let mut m = (l + r) / 2;
            let total = m * (m + 1) * (m + 2) / 6;
            if total > n_i64 {
                r = m;
            } else if total == n_i64 {
                return (m * (m + 1) / 2) as i32;
            } else {
                l = m + 1;
            }
        }

        let mut left_boxes: i64 = n_i64 - (l - 1) * l * (l + 1) / 6;
        //println!("{},{}", l, left_boxes);
        let mut r2: i64 = l + 1;
        let mut l2: i64 = 0;
        while l2 < r2 {
            let mut m = (l2 + r2) / 2;
            let total = m * (m + 1) / 2;
            if total >= left_boxes {
                r2 = m;
            } else {
                l2 = m + 1;
            }
        }

        return (l * (l - 1) / 2 + l2) as i32;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::minimum_boxes(4), 3);
        assert_eq!(Solution::minimum_boxes(10), 6);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::minimum_boxes(11), 7);
        assert_eq!(Solution::minimum_boxes(22), 12);
    }
}

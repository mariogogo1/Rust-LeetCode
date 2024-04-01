/**

887. 鸡蛋掉落
1884. 鸡蛋掉落-两枚鸡蛋

给你 k 枚相同的鸡蛋，并可以使用一栋从第 1 层到第 n 层共有 n 层楼的建筑。

已知存在楼层 f ，满足 0 <= f <= n ，任何从 高于 f 的楼层落下的鸡蛋都会碎，从 f 楼层或比它低的楼层落下的鸡蛋都不会破。

每次操作，你可以取一枚没有碎的鸡蛋并把它从任一楼层 x 扔下（满足 1 <= x <= n）。如果鸡蛋碎了，你就不能再次使用它。如果某枚鸡蛋扔下后没有摔碎，则可以在之后的操作中 重复使用 这枚鸡蛋。

请你计算并返回要确定 f 确切的值 的 最小操作次数 是多少？

提示：

1 <= k <= 100
1 <= n <= 104

https://leetcode.cn/problems/rearranging-fruits/description/
*/
pub struct Solution;

/// 1884 比較簡單可以先練習

/// 直觀想法：
/// 假設dp[i][j]為i層樓，j顆雞蛋的最小操作次數
/// 那麼dp[i][j] = 1 + min (max(dp[i-1][j-1],dp[i-x-1][j])); for all x < i
/// 找min的時間複雜度是O(n)，
/// 要記錄dp[i-x-1][j],dp[i-1][j-1]最大值發生的位置，才能降低時間複雜度，寫起來比較不直觀。
///

/// 換個思路：程式比較容易表達，但需要思考一下
/// 假設dp[i][j]為i次操作，j顆雞蛋的"可以量測"的"最大層數"
/// dp[i][j] =dp[i-1][j-1] + 1  +dp[i-1][j]
///               碎了          沒碎
/// dp[i][j] = 雞蛋少一顆的最大樓層 + 1 + 雞蛋沒碎顆的最大樓層  
///                   |             |              |
///                  底下樓層    在這一樓層丟下雞蛋    往上蓋
///
/// 最後在dp[?][k]中找到第一個 >= n 的下標 i 即是最少操作次數

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let k_usize = k as usize;
        let n_usize = n as usize;
        let mut dp = vec![0; k_usize + 1];
        let mut temp;
        let mut add;
        for i in 0..n_usize {
            add = dp[0];
            for j in 1..=k_usize {
                temp = dp[j];
                dp[j] += add + 1;
                add = temp;
            }
            if dp[k_usize] >= n {
                return (i + 1) as i32;
            }
        }
        unreachable!();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::super_egg_drop(1, 2), 2);
        assert_eq!(Solution::super_egg_drop(2, 6), 3);
        assert_eq!(Solution::super_egg_drop(3, 14), 4);
    }
    #[test]
    fn test_case() {}
}

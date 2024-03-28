/**
1402. 做菜顺序

一个厨师收集了他 n 道菜的满意程度 satisfaction ，这个厨师做出每道菜的时间都是 1 单位时间。

一道菜的 「 like-time 系数 」定义为烹饪这道菜结束的时间（包含之前每道菜所花费的时间）乘以这道菜的满意程度，也就是 time[i]*satisfaction[i] 。

返回厨师在准备了一定数量的菜肴后可以获得的最大 like-time 系数 总和。

你可以按 任意 顺序安排做菜的顺序，你也可以选择放弃做某些菜来获得更大的总和。

提示：

n == satisfaction.length
1 <= n <= 500
-1000 <= satisfaction[i] <= 1000

https://leetcode.cn/problems/reducing-dishes/description/
*/
pub struct Solution;
// 做菜順序
/// 累加到前綴和<0的時候跳出，類似求極值一次微分從大於0變成小於0的地方是極大值
impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_by(|x, y| {
            return y.cmp(x); // 大 -> 小
        });
        let mut ans = 0;
        let mut sum = 0;
        for i in 0..satisfaction.len() {
            sum += satisfaction[i];
            if sum <= 0 {
                break;
            }
            ans += sum;
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]), 14);
        assert_eq!(Solution::max_satisfaction(vec![4, 3, 2]), 20);
        assert_eq!(Solution::max_satisfaction(vec![-1, -4, -5]), 0);
    }
    #[test]
    fn test_case() {}
}

/**
475. 供暖器

冬季已经来临。 你的任务是设计一个有固定加热半径的供暖器向所有房屋供暖。

在加热器的加热半径范围内的每个房屋都可以获得供暖。

现在，给出位于一条水平线上的房屋 houses 和供暖器 heaters 的位置，请你找出并返回可以覆盖所有房屋的最小加热半径。

注意：所有供暖器 heaters 都遵循你的半径标准，加热的半径也一样。

https://leetcode.cn/problems/heaters/description/
*/
pub struct Solution;
impl Solution {
    pub fn find_radius(houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        let mut ans = 0;
        heaters.sort_unstable();

        for house in &houses {
            let idx = heaters.binary_search(&house).unwrap_or_else(|x| x);
            let mut min_dis = i32::MAX;
            if idx < heaters.len() {
                min_dis = heaters[idx] - house;
            }
            if idx > 0 {
                min_dis = min_dis.min(house - heaters[idx - 1]);
            }
            ans = ans.max(min_dis);
        }

        ans
    }
}

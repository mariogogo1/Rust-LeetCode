/**
1276. 不浪费原料的汉堡制作方案
圣诞活动预热开始啦，汉堡店推出了全新的汉堡套餐。为了避免浪费原料，请你帮他们制定合适的制作计划。

给你两个整数 tomatoSlices 和 cheeseSlices，分别表示番茄片和奶酪片的数目。不同汉堡的原料搭配如下：

巨无霸汉堡：4 片番茄和 1 片奶酪
小皇堡：2 片番茄和 1 片奶酪
请你以 [total_jumbo, total_small]（[巨无霸汉堡总数，小皇堡总数]）的格式返回恰当的制作方案，使得剩下的番茄片 tomatoSlices 和奶酪片 cheeseSlices 的数量都是 0。

如果无法使剩下的番茄片 tomatoSlices 和奶酪片 cheeseSlices 的数量为 0，就请返回 []。


https://leetcode.cn/problems/number-of-burgers-with-no-waste-of-ingredients/description/
*/

pub struct Solution;
impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        let mut a = 0;
        let mut b = 0;
        a = (tomato_slices - 2 * cheese_slices) / 2;
        b = (4 * cheese_slices - tomato_slices) / 2;
        if 4 * a + 2 * b != tomato_slices || a + b != cheese_slices || a < 0 || b < 0 {
            return vec![];
        }
        vec![a, b]
    }
}
